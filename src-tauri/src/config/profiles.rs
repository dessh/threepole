use std::collections::HashSet;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::ConfigFile;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Profiles {
    pub saved_profiles: HashSet<Profile>,
    pub selected_profile: Option<Profile>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub account_platform: usize,
    pub account_id: String,
}

impl ConfigFile for Profiles {
    fn get_filename() -> &'static str {
        "profiles.json"
    }
}
