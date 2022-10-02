use serde::{Deserialize, Serialize};

use super::ConfigFile;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Profiles {
    pub saved_profiles: Vec<Profile>,
    pub selected_profile: Option<Profile>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub account_platform: usize,
    pub account_id: String,
    pub display_name: String,
    pub display_tag: usize,
}

impl ConfigFile for Profiles {
    fn get_filename() -> &'static str {
        "profiles.json"
    }
}
