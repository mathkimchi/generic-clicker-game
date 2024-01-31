use gloo::storage::{LocalStorage, Storage};
use gloo_console::log;

use crate::upgrades::{Upgrade, UpgradeItem};

pub fn tick_logic() {
    // 1 tick = 1 second for now

    // TODO: abstract this
    // figure out to go bottom up or top bottom

    let points_per_tick_per_tick =
        LocalStorage::get("generic-clicker-game.helper2s").unwrap_or(0i128);

    let old_points_per_tick = LocalStorage::get("generic-clicker-game.helpers").unwrap_or(0i128);

    let new_points_per_tick = old_points_per_tick + points_per_tick_per_tick;

    let _ = LocalStorage::set("generic-clicker-game.helpers", new_points_per_tick);

    let old_points = LocalStorage::get("generic-clicker-game.points").unwrap_or(0i128);

    let new_points = old_points + new_points_per_tick;

    let _ = LocalStorage::set("generic-clicker-game.points", new_points);
}

pub fn buy_upgrade(upgrade: &Upgrade) {
    // for each upgrade item, check if they will be non-negative after the upgrade (intended for the negative changes but generalized)
    let possible = &upgrade.items.iter().all(
        |UpgradeItem {
             display_name: _,
             storage_key,
             change,
         }| {
            let current = LocalStorage::get(storage_key).unwrap_or(0i128);
            let new = current + *change;
            new >= 0
        },
    );

    if !possible {
        log!("Too poor error upgrade: ", format!("{}", upgrade));
        return;
    }

    for UpgradeItem {
        display_name: _,
        storage_key,
        change,
    } in &upgrade.items
    {
        let current = LocalStorage::get(storage_key).unwrap_or(0i128);
        let new = current + *change;
        let _ = LocalStorage::set(storage_key, new);
    }
}
