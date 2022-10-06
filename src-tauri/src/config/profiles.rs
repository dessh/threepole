use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};

use super::ConfigFile;

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Profiles {
    pub saved_profiles: Vec<Profile>,
    pub selected_profile: Option<Profile>,
}

impl<'de> Deserialize<'de> for Profiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Profiles {
            saved_profiles: Vec<Profile>,
            selected_profile: Option<Profile>,
        }

        let profiles = _Profiles::deserialize(deserializer)?;
        Ok(Self {
            saved_profiles: profiles.saved_profiles.into_iter().unique().collect(),
            selected_profile: profiles.selected_profile,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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
