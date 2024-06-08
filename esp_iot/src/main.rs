use std::time::Duration;

use buttplug::{
    client::{ButtplugClient, ScalarValueCommand},
    core::{
        connector::new_json_ws_client_connector, message::ClientGenericDeviceMessageAttributes,
    },
};
use pb::{apexlegends::EspDataOption, esp_service::esp_service_client::EspServiceClient};
use tokio::time::sleep;

mod pb;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let esp_server_addr = "http://[::1]:50051";
    let device_connector = new_json_ws_client_connector("ws://localhost:12345");

    let (mut esp_client, device_client) = tokio::try_join!(
        async move {
            let client = EspServiceClient::connect(esp_server_addr)
                .await?
                .accept_compressed(tonic::codec::CompressionEncoding::Zstd)
                .send_compressed(tonic::codec::CompressionEncoding::Zstd);
            anyhow::Ok(client)
        },
        async move {
            let client = ButtplugClient::new("Default Client");
            client.connect(device_connector).await?;
            anyhow::Ok(client)
        }
    )?;

    println!("Connected!");

    device_client.start_scanning().await?;
    loop {
        sleep(Duration::from_secs(2)).await;

        println!("Client currently knows about these devices:");
        for device in device_client.devices() {
            println!("- {}", device.name());
        }
        println!();

        if !device_client.devices().is_empty() {
            break;
        }
    }
    device_client.stop_scanning().await?;

    let device = &device_client.devices()[0];

    {
        fn print_attrs(attrs: &Vec<ClientGenericDeviceMessageAttributes>) {
            for attr in attrs {
                println!(
                    "{}: {} - Steps: {}",
                    attr.actuator_type(),
                    attr.feature_descriptor(),
                    attr.step_count()
                );
            }
        }
        println!("{} supports these actions:", device.name());
        if let Some(attrs) = device.message_attributes().scalar_cmd() {
            print_attrs(attrs);
        }
        print_attrs(&device.rotate_attributes());
        print_attrs(&device.linear_attributes());
        println!("Battery: {}", device.has_battery_level());
        println!("RSSI: {}", device.has_rssi_level());
    }

    loop {
        let esp_data = esp_client
            .get_esp_data(EspDataOption {
                version: 0,
                full_aimbot_state: false,
                full_targets_list: false,
            })
            .await?
            .into_inner();

        if !esp_data.ready {
            println!("ESP service not ready!");
            break;
        }

        if !esp_data.in_game || !esp_data.local_player.is_some() {
            println!("Waiting for the game to be ready..");
            sleep(Duration::from_secs(2)).await;
            continue;
        }

        let player = esp_data.local_player.as_ref().unwrap();

        let shield_health = player.shield + player.temp_shield_health + player.extra_shield_health;

        println!(
            "player: {}, HP: {}, shield: {}, damage: {}, kills: {}",
            player.player_name, player.health, shield_health, player.damage_dealt, player.kills
        );

        let shield_level = match shield_health {
            0..25 => 1,
            25..50 => 2,
            50..75 => 3,
            75..100 => 4,
            100..125 => 5,
            _ => -1,
        };
        let vibrate_level = match shield_level {
            1..=5 => (5.0 - shield_level as f64) / 5.0,
            _ => 0.0,
        };

        device
            .vibrate(&ScalarValueCommand::ScalarValue(vibrate_level))
            .await?;

        sleep(Duration::from_secs(1)).await;
    }

    device_client.disconnect().await?;

    Ok(())
}
