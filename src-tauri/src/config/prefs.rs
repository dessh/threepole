use serde::{Deserialize, Serialize};

use super::ConfigFile;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Prefs {
    pub display_daily_clears: bool,
    pub display_clear_notifications: bool,
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            display_daily_clears: true,
            display_clear_notifications: true,
        }
    }
}

impl ConfigFile for Prefs {
    fn get_filename() -> &'static str {
        "preferences.json"
    }
}
