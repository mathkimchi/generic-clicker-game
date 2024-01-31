use std::fmt::Display;

/// For an explanation of what these are, look at the examples
#[derive(Debug, Clone)]
pub struct UpgradeItem {
    pub display_name: String,
    pub storage_key: String,
    pub change: i32,
}

#[derive(Debug, Clone)]
pub struct Upgrade {
    pub upgrade_name: String,
    pub items: Vec<UpgradeItem>,
}
impl Display for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = f.write_str(&self.upgrade_name);
        result = result.and(f.write_str(" ("));

        for (index, item) in self.items.iter().enumerate() {
            if index > 0 {
                result = result.and(f.write_str(", "));
            }

            if item.change > 0 {
                result = result.and(f.write_str("+"));
            }
            result = result.and(write!(f, "{}", item.change));
            result = result.and(f.write_str(&item.display_name));
        }

        result = result.and(f.write_str(")"));

        result
    }
}

// pub static UPGRADE_LIST: Vec<Upgrade> = vec![Upgrade {
//     upgrade_name: "Click".to_string(),
//     items: vec![],
// }];

pub fn get_upgrade_list() -> Vec<Upgrade> {
    vec![
        Upgrade {
            upgrade_name: "Increment".to_string(),
            items: vec![UpgradeItem {
                display_name: "pts".to_string(),
                storage_key: "generic-clicker-game.points".to_string(),
                change: 1,
            }],
        },
        Upgrade {
            upgrade_name: "Buy Helper".to_string(),
            items: vec![
                UpgradeItem {
                    display_name: "pts/sec".to_string(),
                    storage_key: "generic-clicker-game.helpers".to_string(),
                    change: 1,
                },
                UpgradeItem {
                    display_name: "pts".to_string(),
                    storage_key: "generic-clicker-game.points".to_string(),
                    change: -2,
                },
            ],
        },
    ]
}
