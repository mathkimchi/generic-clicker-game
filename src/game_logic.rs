use gloo::storage::{LocalStorage, Storage};
use gloo_console::log;

use crate::upgrades::{Upgrade, UpgradeItem};

pub fn tick_logic() {
    // 1 tick = 1 second for now

    let points_per_tick = LocalStorage::get("generic-clicker-game.helpers").unwrap_or(0);

    let old_points = LocalStorage::get("generic-clicker-game.points").unwrap_or(0);

    let new_points = old_points + points_per_tick;

    let _ = LocalStorage::set("generic-clicker-game.points", new_points);
}

pub fn increment() {
    let old_points = LocalStorage::get("generic-clicker-game.points").unwrap_or(0);
    let _ = LocalStorage::set("generic-clicker-game.points", old_points + 1);
}

pub fn buy_helper() {
    // TODO: abstract later

    let old_points = LocalStorage::get("generic-clicker-game.points").unwrap_or(0);

    if old_points >= 2 {
        let _ = LocalStorage::set("generic-clicker-game.points", old_points - 2);
        let old_helpers = LocalStorage::get("generic-clicker-game.helpers").unwrap_or(0);
        let _ = LocalStorage::set("generic-clicker-game.helpers", old_helpers + 1);
    } else {
        log!("Too poor, can't buy (need at least 2 points)");
    }
}

pub fn buy_upgrade(upgrade: &Upgrade) {
    // for each upgrade item, check if they will be non-negative after the upgrade (intended for the negative changes but generalized)
    let possible = &upgrade.items.iter().all(
        |UpgradeItem {
             display_name: _,
             storage_key,
             change,
         }| {
            let current = LocalStorage::get(storage_key).unwrap_or(0);
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
        let current = LocalStorage::get(storage_key).unwrap_or(0);
        let new = current + *change;
        let _ = LocalStorage::set(storage_key, new);
    }
}
