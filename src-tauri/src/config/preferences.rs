use serde::{Deserialize, Serialize};

use super::ConfigFile;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Preferences {
    pub enable_overlay: bool,
    pub display_daily_clears: bool,
    pub display_clear_notifications: bool,
    pub display_milliseconds: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            enable_overlay: true,
            display_daily_clears: true,
            display_clear_notifications: true,
            display_milliseconds: true,
        }
    }
}

impl ConfigFile for Preferences {
    fn get_filename() -> &'static str {
        "preferences.json"
    }
}
