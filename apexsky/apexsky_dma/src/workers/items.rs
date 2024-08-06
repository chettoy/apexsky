use std::time::Duration;

use apexsky::{config::Settings, global_state::G_STATE};
use apexsky_proto::pb::apexlegends::TreasureClue;
use obfstr::obfstr as s;
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio::time::{sleep_until, Instant};
use tracing::instrument;

use crate::game::data::*;
use crate::SharedStateWrapper;

#[instrument]
pub async fn items_loop(
    mut active: watch::Receiver<bool>,
    shared_state: SharedStateWrapper,
    items_glow_tx: watch::Sender<Vec<(u64, u8)>>,
) -> anyhow::Result<()> {
    let mut start_instant = Instant::now();

    tracing::debug!("{}", s!("task start"));

    while *active.borrow_and_update() {
        start_instant += Duration::from_millis(500);
        sleep_until(start_instant).await;
        start_instant = Instant::now();

        if shared_state.get_game_baseaddr().is_none() || !shared_state.is_world_ready() {
            tracing::trace!("{}", s!("waiting for world ready"));
            continue;
        }

        let g_settings = G_STATE.lock().unwrap().config.settings.clone();
        let treasure_clues = shared_state.treasure_clues.read().clone();

        let item_glow = treasure_clues
            .into_values()
            .filter_map(|clue| {
                process_loot(&clue, &g_settings).map(|glow_ctx| (clue.entity_handle, glow_ctx))
            })
            .collect();

        items_glow_tx.send(item_glow).unwrap_or_else(|e| {
            tracing::error!(%e, ?items_glow_tx, "{}", s!("send item glow"));
        });
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}

#[instrument(skip_all, fields(clue))]
fn process_loot(clue: &TreasureClue, g_settings: &Settings) -> Option<u8> {
    let ptr = clue.entity_handle;
    if ptr <= 0 {
        tracing::error!(?clue);
        return None;
    }

    if clue.distance > g_settings.aimbot_settings.aim_dist {
        return None;
    }

    let select = &g_settings.loot;

    match ItemId(clue.item_id) {
        // DeathBox
        ItemId::ApexskyItemDeathBox if g_settings.deathbox => Some(HIGHLIGHT_DEATH_BOX),

        // Backpacks
        ItemId::LightBackpack if select.lightbackpack => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::MedBackpack if select.medbackpack => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::HeavyBackpack if select.heavybackpack => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::GoldBackpack if select.goldbackpack => Some(HIGHLIGHT_LOOT_GOLD),

        // Shields
        ItemId::ArmorCore1 if select.shieldupgrade1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::ArmorCore2 if select.shieldupgrade2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::ArmorCore3 if select.shieldupgrade3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::ArmorCore4 if select.shieldupgrade5 => Some(HIGHLIGHT_LOOT_RED),
        ItemId::ShieldUpgradeHead1 if select.shieldupgradehead1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::ShieldUpgradeHead2 if select.shieldupgradehead2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::ShieldUpgradeHead3 if select.shieldupgradehead3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::ShieldUpgradeHead4 if select.shieldupgradehead4 => Some(HIGHLIGHT_LOOT_GOLD),

        // Heals
        ItemId::Accelerant if select.accelerant => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::Phoenix if select.phoenix => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::HealthLarge if select.healthlarge => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::HealthSmall if select.healthsmall => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::ShieldBatterySmall if select.shieldbattsmall => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::ShieldBatteryLarge if select.shieldbattlarge => Some(HIGHLIGHT_LOOT_BLUE),

        // Ammos
        ItemId::LightAmmo if select.lightammo => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::HeavyAmmo if select.heavyammo => Some(HIGHLIGHT_LOOT_HEAVY),
        ItemId::EnergyAmmo if select.energyammo => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::SniperAmmo if select.sniperammo => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::ShotgunAmmo if select.shotgunammo => Some(HIGHLIGHT_LOOT_RED),

        // Mags
        ItemId::LightAmmoMag1 if select.lightammomag1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::LightAmmoMag2 if select.lightammomag2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::LightAmmoMag3 if select.lightammomag3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::LightAmmoMag4 if select.lightammomag4 => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::HeavyAmmoMag1 if select.heavyammomag1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::HeavyAmmoMag2 if select.heavyammomag2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::HeavyAmmoMag3 if select.heavyammomag3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::HeavyAmmoMag4 if select.heavyammomag4 => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::SniperAmmoMag1 if select.sniperammomag1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::SniperAmmoMag2 if select.sniperammomag2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::SniperAmmoMag3 if select.sniperammomag3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::SniperAmmoMag4 if select.sniperammomag4 => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::EnergyAmmoMag1 if select.energyammomag1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::EnergyAmmoMag2 if select.energyammomag2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::EnergyAmmoMag3 if select.energyammomag3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::EnergyAmmoMag4 if select.energyammomag4 => Some(HIGHLIGHT_LOOT_GOLD),

        // Stocks
        ItemId::StockSniper1 if select.stocksniper1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::StockSniper2 if select.stocksniper2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::StockSniper3 if select.stocksniper3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::StockRegular1 if select.stockregular1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::StockRegular2 if select.stockregular2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::StockRegular3 if select.stockregular3 => Some(HIGHLIGHT_LOOT_PURPLE),

        // Down Shields
        ItemId::ShieldDown1 if select.shielddown1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::ShieldDown2 if select.shielddown2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::ShieldDown3 if select.shielddown3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::ShieldDown4 if select.shielddown4 => Some(HIGHLIGHT_LOOT_GOLD),

        // Optics
        ItemId::Optic1xHCOG if select.optic1xhcog => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::Optic2xHCOG if select.optic2xhcog => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::OpticHolo1x if select.opticholo1x => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::OpticHolo1x2x if select.opticholo1x2x => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::OpticThreat if select.opticthreat => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::Optic3xHCOG if select.optic3xhcog => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::Optic2x4x if select.optic2x4x => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::OpticSniper6x if select.opticsniper6x => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::OpticSniper4x8x if select.opticsniper4x8x => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::OpticSniperThreat if select.opticsniperthreat => Some(HIGHLIGHT_LOOT_GOLD),

        // Hop-ups
        ItemId::LaserSight1 if select.lasersight1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::LaserSight2 if select.lasersight2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::LaserSight3 if select.lasersight3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::Suppressor1 if select.suppressor1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::Suppressor2 if select.suppressor2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::Suppressor3 if select.suppressor3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::TurboCharger if select.turbo_charger => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::SkullPiecer if select.skull_piecer => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::HammerPoint if select.hammer_point => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::DisruptorRounds if select.disruptor_rounds => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::BoostedLoader if select.boosted_loader => Some(HIGHLIGHT_LOOT_GOLD),
        ItemId::ShotgunBolt1 if select.shotgunbolt1 => Some(HIGHLIGHT_LOOT_WHITE),
        ItemId::ShotgunBolt2 if select.shotgunbolt2 => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::ShotgunBolt3 if select.shotgunbolt3 => Some(HIGHLIGHT_LOOT_PURPLE),
        ItemId::ShotgunBolt4 if select.shotgunbolt4 => Some(HIGHLIGHT_LOOT_GOLD),

        // Nades
        ItemId::GrenadeFrag if select.grenade_frag => Some(HIGHLIGHT_LOOT_RED),
        ItemId::GrenadeThermite if select.grenade_thermite => Some(HIGHLIGHT_LOOT_RED),
        ItemId::GrenadeArcStar if select.grenade_arc_star => Some(HIGHLIGHT_LOOT_GREY),

        // Weapons
        ItemId::WeaponKraber if select.weapon_kraber => Some(HIGHLIGHT_LOOT_RED),
        ItemId::WeaponMastiff if select.weapon_mastiff => Some(HIGHLIGHT_LOOT_RED),
        ItemId::WeaponLStar if select.weapon_lstar => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::WeaponNemesis if select.weapon_nemesis => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::WeaponHavoc if select.weapon_havoc => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::WeaponDevotion if select.weapon_devotion => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::WeaponTripleTake if select.weapon_triple_take => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::WeaponFlatline if select.weapon_flatline => Some(HIGHLIGHT_LOOT_HEAVY),
        ItemId::WeaponHemlock if select.weapon_hemlock => Some(HIGHLIGHT_LOOT_HEAVY),
        ItemId::WeaponG7Scout if select.weapon_g7_scout => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponAlternator if select.weapon_alternator => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponR99 if select.weapon_r99 => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponProwler if select.weapon_prowler => Some(HIGHLIGHT_LOOT_HEAVY),
        ItemId::WeaponVolt if select.weapon_volt => Some(HIGHLIGHT_LOOT_ENERGY),
        ItemId::WeaponLongbow if select.weapon_longbow => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::WeaponChargeRifle if select.weapon_charge_rifle => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::WeaponSpitfire if select.weapon_spitfire => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponR301 if select.weapon_r301 => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponEva8 if select.weapon_eva8 => Some(HIGHLIGHT_LOOT_RED),
        ItemId::WeaponPeacekeeper if select.weapon_peacekeeper => Some(HIGHLIGHT_LOOT_RED),
        ItemId::WeaponMozambique if select.weapon_mozambique => Some(HIGHLIGHT_LOOT_RED),
        ItemId::WeaponWingman if select.weapon_wingman => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::WeaponP2020 if select.weapon_p2020 => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponRE45 if select.weapon_re45 => Some(HIGHLIGHT_LOOT_LIGHT),
        ItemId::WeaponSentinel if select.weapon_sentinel => Some(HIGHLIGHT_LOOT_BLUE),
        ItemId::WeaponBow if select.weapon_bow => Some(HIGHLIGHT_LOOT_RED),
        ItemId::Weapon3030Repeater if select.weapon_3030_repeater => Some(HIGHLIGHT_LOOT_HEAVY),
        ItemId::WeaponRampage if select.weapon_rampage => Some(HIGHLIGHT_LOOT_HEAVY),
        ItemId::WeaponCARSMG if select.weapon_car_smg => Some(HIGHLIGHT_LOOT_HEAVY),

        _ => None,
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct LootInt {
    pub(crate) int: i32,
    pub(crate) model: String,
}

pub fn export_new_items(loots: Vec<LootInt>) -> anyhow::Result<()> {
    use std::fs;
    use std::io::Write;

    let mut modify = false;
    for item in &loots {
        let Some(model) = ITEM_LIST.get(&item.int) else {
            modify = true;
            tracing::info!(?item, "{}", s!("new loot item"));
            continue;
        };
        if *model != item.model {
            modify = true;
            //tracing::info!(?item, "{}", s!("loot model changed"));
        }
    }

    if modify {
        let items_json = serde_json::to_string(&loots)?;
        let path = std::env::current_dir()?.join(s!("updated_item.json"));
        let mut json_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        write!(json_file, "{}", items_json)?;
    }

    Ok(())
}
