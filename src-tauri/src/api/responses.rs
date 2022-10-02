use std::{cmp::Ordering, collections::HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyMembership {
    membership_type: usize,
    membership_id: String,
    bungie_global_display_name: String,
    bungie_global_display_name_code: usize,
    icon_path: String,
}

#[derive(Debug)]
pub struct CharacterCurrentActivities {
    pub privacy: usize,
    pub activities: Option<Vec<CharacterCurrentActivity>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CharacterCurrentActivity {
    pub character_id: String,
    pub date_activity_started: DateTime<Utc>,
    pub current_activity_hash: usize,
}

impl PartialOrd for CharacterCurrentActivity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date_activity_started
            .partial_cmp(&other.date_activity_started)
    }
}

impl Ord for CharacterCurrentActivity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date_activity_started.cmp(&other.date_activity_started)
    }
}

impl<'de> Deserialize<'de> for CharacterCurrentActivities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Profile {
            character_activities: CurrentActivities,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CurrentActivities {
            data: Option<HashMap<String, CurrentActivity>>,
            privacy: usize,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CurrentActivity {
            date_activity_started: DateTime<Utc>,
            current_activity_hash: usize,
        }

        let profile = Profile::deserialize(deserializer)?;
        Ok(Self {
            privacy: profile.character_activities.privacy,
            activities: profile.character_activities.data.map(|d| {
                d.iter()
                    .map(|e| CharacterCurrentActivity {
                        character_id: e.0.to_string(),
                        date_activity_started: e.1.date_activity_started,
                        current_activity_hash: e.1.current_activity_hash,
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

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub struct CompletedActivity {
    pub period: DateTime<Utc>,
    pub instance_id: String,
    pub completed: bool,
    pub activity_duration: String,
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
        struct Activity {
            period: DateTime<Utc>,
            activity_details: ActivityDetails,
            values: Values,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ActivityDetails {
            instance_id: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Values {
            completion_reason: Value,
            completed: Value,
            activity_duration_seconds: Value,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Value {
            basic: BasicValue,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct BasicValue {
            value: f32,
            display_value: String,
        }

        let activity = Activity::deserialize(deserializer)?;
        Ok(Self {
            period: activity.period,
            instance_id: activity.activity_details.instance_id,
            completed: activity.values.completed.basic.value == 1.0
                && activity.values.completion_reason.basic.value == 0.0,
            activity_duration: activity
                .values
                .activity_duration_seconds
                .basic
                .display_value,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyActivityDefinition {
    pub activity_type_hash: usize,
}
