use std::{cmp::Ordering, collections::HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::consts::{RAID_ACTIVITY_HASH, RAID_ACTIVITY_MODE};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BungieProfile {
    membership_type: usize,
    membership_id: String,
    bungie_global_display_name: String,
    bungie_global_display_name_code: usize,
    cross_save_override: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfo {
    pub privacy: usize,
    pub display_name: String,
    pub display_tag: usize,
    pub character_ids: Vec<String>,
}

impl<'de> Deserialize<'de> for ProfileInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Profile {
            profile: _ProfileInfo,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _ProfileInfo {
            data: _ProfileData,
            privacy: usize,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _ProfileData {
            user_info: _UserInfo,
            character_ids: Vec<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _UserInfo {
            bungie_global_display_name: String,
            bungie_global_display_name_code: usize,
        }

        let profile = _Profile::deserialize(deserializer)?;
        Ok(Self {
            privacy: profile.profile.privacy,
            display_name: profile.profile.data.user_info.bungie_global_display_name,
            display_tag: profile
                .profile
                .data
                .user_info
                .bungie_global_display_name_code,
            character_ids: profile.profile.data.character_ids,
        })
    }
}

#[derive(Debug)]
pub struct ProfileCurrentActivities {
    pub privacy: usize,
    pub activities: Option<HashMap<String, LatestCharacterActivity>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LatestCharacterActivity {
    pub date_activity_started: DateTime<Utc>,
    pub current_activity_hash: usize,
}

impl PartialOrd for LatestCharacterActivity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date_activity_started
            .partial_cmp(&other.date_activity_started)
    }
}

impl Ord for LatestCharacterActivity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date_activity_started.cmp(&other.date_activity_started)
    }
}

impl<'de> Deserialize<'de> for ProfileCurrentActivities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Profile {
            character_activities: _CurrentActivities,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _CurrentActivities {
            data: Option<HashMap<String, _CurrentActivity>>,
            privacy: usize,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _CurrentActivity {
            date_activity_started: DateTime<Utc>,
            current_activity_hash: usize,
        }

        let profile = _Profile::deserialize(deserializer)?;
        Ok(Self {
            privacy: profile.character_activities.privacy,
            activities: profile.character_activities.data.map(|d| {
                d.into_iter()
                    .map(|e| {
                        (
                            e.0,
                            LatestCharacterActivity {
                                date_activity_started: e.1.date_activity_started,
                                current_activity_hash: e.1.current_activity_hash,
                            },
                        )
                    })
                    .collect()
            }),
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterActivityHistory {
    pub activities: Option<Vec<CompletedActivity>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompletedActivity {
    pub period: DateTime<Utc>,
    pub instance_id: String,
    pub activity_hash: usize,
    pub modes: Vec<usize>,
    pub completed: bool,
    pub activity_duration: String,
    pub activity_duration_seconds: usize,
}

impl PartialOrd for CompletedActivity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.period.partial_cmp(&other.period)
    }
}

impl Ord for CompletedActivity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.period.cmp(&other.period)
    }
}

impl<'de> Deserialize<'de> for CompletedActivity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Activity {
            period: DateTime<Utc>,
            activity_details: _ActivityDetails,
            values: _Values,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _ActivityDetails {
            instance_id: String,
            director_activity_hash: usize,
            modes: Vec<usize>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Values {
            completion_reason: _Value,
            completed: _Value,
            activity_duration_seconds: _Value,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Value {
            basic: _BasicValue,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _BasicValue {
            value: f32,
            display_value: String,
        }

        let activity = _Activity::deserialize(deserializer)?;
        Ok(Self {
            period: activity.period,
            instance_id: activity.activity_details.instance_id,
            activity_hash: activity.activity_details.director_activity_hash,
            modes: activity.activity_details.modes,
            completed: activity.values.completed.basic.value == 1.0
                && activity.values.completion_reason.basic.value == 0.0,
            activity_duration: activity
                .values
                .activity_duration_seconds
                .basic
                .display_value,
            activity_duration_seconds: activity.values.activity_duration_seconds.basic.value
                as usize,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActivityInfo {
    pub name: String,
    pub activity_modes: Vec<usize>,
    pub background_image: Option<String>,
}

impl<'de> Deserialize<'de> for ActivityInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Activity {
            original_display_properties: _DisplayProperties,
            activity_mode_types: Option<Vec<usize>>,
            activity_type_hash: usize,
            pgcr_image: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _DisplayProperties {
            name: String,
        }

        // No activity modes in raid definitions :(
        fn modes_from_hash(hash: usize) -> Vec<usize> {
            let mut v = vec![];

            if hash == RAID_ACTIVITY_HASH {
                v.push(RAID_ACTIVITY_MODE);
            }

            v
        }

        let activity = _Activity::deserialize(deserializer)?;
        Ok(Self {
            name: activity.original_display_properties.name,
            activity_modes: activity
                .activity_mode_types
                .unwrap_or_else(|| modes_from_hash(activity.activity_type_hash)),
            background_image: activity.pgcr_image,
        })
    }
}
