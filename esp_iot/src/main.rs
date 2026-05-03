use std::time::Duration;

use anyhow::Context as _;
use apex1_common::pb::{
    apexlegends::EspDataOption, esp_service::esp_service_client::EspServiceClient,
};
use buttplug_client::{
    ButtplugClient, ButtplugClientDevice, ButtplugClientError, ButtplugClientEvent,
    connector::ButtplugRemoteClientConnector, device::ClientDeviceOutputCommand,
    serializer::ButtplugClientJSONSerializer,
};
use buttplug_core::message::OutputType;
use buttplug_transport_websocket_tungstenite::ButtplugWebsocketClientTransport;
use futures::stream::StreamExt;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let esp_server_addr = "http://[::1]:50051";
    let connector = ButtplugRemoteClientConnector::<
        ButtplugWebsocketClientTransport,
        ButtplugClientJSONSerializer,
    >::new(ButtplugWebsocketClientTransport::new_insecure_connector(
        "ws://localhost:12345",
    ));

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

            client
                .connect(connector)
                .await
                .inspect_err(|e| match e {
                    ButtplugClientError::ButtplugConnectorError(error) => {
                        println!("ERROR: Could not connect to Intiface Central!");
                        println!(
                            "Make sure Intiface Central is running and the server is started."
                        );
                        println!("Default address: ws://127.0.0.1:12345");
                        println!("Error: {}", error);
                    }
                    _ => (),
                })
                .context("Can't connect to Buttplug Server")?;
            anyhow::Ok(client)
        }
    )?;

    let mut events = device_client.event_stream();

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            match event {
                ButtplugClientEvent::DeviceAdded(device) => {
                    println!("[+] Device connected: {}", device.name());
                }
                ButtplugClientEvent::DeviceRemoved(info) => {
                    println!("[-] Device disconnected: {}", info.name());
                }
                ButtplugClientEvent::ServerDisconnect => {
                    println!("[!] Server connection lost!");
                }
                ButtplugClientEvent::Error(err) => {
                    println!("[!] Error: {}", err);
                }
                _ => {}
            }
        }
    });

    println!("Scanning for devices...");
    println!("Turn on your Bluetooth/USB devices now.");
    println!();

    device_client.start_scanning().await?;
    loop {
        sleep(Duration::from_secs(2)).await;

        println!("Client currently knows about these devices:");
        for (_, device) in device_client.devices() {
            println!("- {}", device.name());
        }
        println!();

        if !device_client.devices().is_empty() {
            break;
        }
    }
    device_client.stop_scanning().await?;

    // Display device capabilities

    let devices: Vec<ButtplugClientDevice> = device_client.devices().into_values().collect();

    for device in &devices {
        print_device_capabilities(device);
    }

    let device = devices.first().unwrap();

    loop {
        let esp_data = esp_client
            .get_esp_data(EspDataOption {
                version: 0,
                full_aimbot_state: false,
                full_targets_list: false,
                sync: false,
            })
            .await?
            .into_inner();

        if !esp_data.ready {
            println!("ESP service not ready!");
            break;
        }

        if !esp_data.in_game || esp_data.local_player.is_none() {
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

        // if !device.output_available(OutputType::Vibrate) {
        match device
            .run_output(&ClientDeviceOutputCommand::Vibrate(vibrate_level.into()))
            .await
        {
            Ok(_) => println!(
                "  {}: vibrating at {:.1}%",
                device.name(),
                vibrate_level * 100.0
            ),
            Err(e) => println!("  {}: error - {}", device.name(), e),
        }
        // }

        sleep(Duration::from_secs(1)).await;
    }

    device_client.stop_all_devices().await?;
    device_client.disconnect().await?;

    Ok(())
}

fn print_device_capabilities(device: &ButtplugClientDevice) {
    println!("  {}", device.name());

    // Check output capabilities (things we can make the device do)
    let mut outputs = Vec::new();
    if device.output_available(OutputType::Vibrate) {
        outputs.push("Vibrate");
    }
    /*
    if !device.rotate_features().is_empty() {
      outputs.push("Rotate");
    }
    if !device.oscillate_features().is_empty() {
      outputs.push("Oscillate");
    }
    if !device.position_features().is_empty() {
      outputs.push("Position");
    }
    */

    if !outputs.is_empty() {
        println!("    Outputs: {}", outputs.join(", "));
    }

    // Check input capabilities (sensors we can read)
    let mut inputs = Vec::new();
    if device.input_available(buttplug_core::message::InputType::Battery) {
        inputs.push("Battery");
    }
    if device.input_available(buttplug_core::message::InputType::Rssi) {
        inputs.push("RSSI");
    }

    if !inputs.is_empty() {
        println!("    Inputs: {}", inputs.join(", "));
    }

    println!();
}
