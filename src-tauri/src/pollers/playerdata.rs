use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tauri::{
    async_runtime::{self, JoinHandle},
    AppHandle, Manager,
};
use tokio::sync::Mutex;

use crate::{
    api::{
        requests::BungieResponseError,
        responses::{ActivityInfo, CompletedActivity, LatestCharacterActivity},
        Api, ApiError, Source,
    },
    config::profiles::Profile,
    consts::API_POLL_INTERVAL,
    ConfigContainer,
};

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    current_activity: Option<CurrentActivity>,
    activity_history: Vec<CompletedActivity>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CurrentActivity {
    start_date: DateTime<Utc>,
    activity_info: ActivityInfo,
}

#[derive(Default)]
pub struct PlayerDataPoller {
    task_handle: Option<JoinHandle<()>>,
    current_playerdata: Option<Arc<Mutex<PlayerData>>>,
    // TODO: maybe add preferences/profiles the poller was started on to see if it needs a reset on change
    // think about reset ting
}

impl PlayerDataPoller {
    pub fn reset(&mut self, app_handle: AppHandle) {
        if let Some(t) = self.task_handle.as_ref() {
            t.abort();
        }

        let current_playerdata = Arc::new(Mutex::new(PlayerData::default()));

        let playerdata_clone = current_playerdata.clone();

        self.current_playerdata = Some(current_playerdata);
        send_blank_update(&app_handle);

        self.task_handle = Some(async_runtime::spawn(async move {
            let profile_opt = {
                let container = app_handle.state::<ConfigContainer>();
                let lock = container.0.lock().await;
                lock.get_profiles().selected_profile.clone()
            };

            let profile = match profile_opt {
                Some(p) => p,
                None => {
                    send_data_update(&app_handle, Err("No profile set".to_string()));
                    return;
                }
            };

            {
                let mut lock = playerdata_clone.lock().await;

                if let Err(e) =
                    update_current(&app_handle, &mut lock.current_activity, &profile).await
                {
                    send_data_update(&app_handle, Err(e.to_string()));
                } else if let Err(e) =
                    update_history(&app_handle, &mut lock.activity_history, &profile).await
                {
                    send_data_update(&app_handle, Err(e.to_string()));
                } else {
                    send_data_update(&app_handle, Ok(lock.clone()));
                }
            }

            let mut count = 0;

            loop {
                tokio::time::sleep(API_POLL_INTERVAL).await;

                {
                    let mut lock = playerdata_clone.lock().await;

                    let res = if count < 5 {
                        update_current(&app_handle, &mut lock.current_activity, &profile).await
                    } else {
                        count = 0;
                        update_history(&app_handle, &mut lock.activity_history, &profile).await
                    };

                    match res {
                        Ok(true) => send_data_update(&app_handle, Ok(lock.clone())),
                        Err(e) => send_data_update(&app_handle, Err(e.to_string())),
                        _ => (),
                    }
                }

                count += 1;
            }
        }));
    }

    // For overlay to get initial data faster during poll timeout
    pub fn get_data(&mut self) -> Option<PlayerData> {
        return match &self.current_playerdata {
            Some(m) => match m.try_lock() {
                Ok(l) => Some(l.clone()),
                Err(_) => None, // If lock currently in use, meaning stat update is in progress
            },
            None => None, // If lock doesn't exist, meaning poller isn't initialized
        };
    }
}

fn send_blank_update(handle: &AppHandle) {
    if let Some(o) = handle.get_window("overlay") {
        o.emit("playerdata_update", None::<Result<PlayerData, String>>)
            .unwrap();
    }
}

fn send_data_update(handle: &AppHandle, data: Result<PlayerData, String>) {
    if let Some(o) = handle.get_window("overlay") {
        o.emit("playerdata_update", data).unwrap();
    }
}

async fn update_current(
    handle: &AppHandle,
    last_activity: &mut Option<CurrentActivity>,
    profile: &Profile,
) -> Result<bool> {
    let current_activities = Api::get_profile_activities(profile).await?;

    let activities = match current_activities.activities {
        Some(a) => a,
        None => bail!("Profile is private"),
    };

    let (characters, activities): (Vec<String>, Vec<LatestCharacterActivity>) =
        activities.into_iter().unzip();

    let latest_activity = activities
        .into_iter()
        .max()
        .ok_or(anyhow!("No character data for profile"))?;

    if let Some(a) = last_activity {
        if a.start_date > latest_activity.date_activity_started {
            // not >=, because it's possible for API to respond with new date but incorrect activity
            return Ok(false);
        }
    }

    let api = handle.state::<Api>();

    api.profile_info_source
        .lock()
        .await
        .set_characters(profile, characters);

    if latest_activity.current_activity_hash == 0 {
        *last_activity = None;
        return Ok(true);
    }

    let current_activity_info = {
        let activity = api
            .activity_info_source
            .lock()
            .await
            .get(&latest_activity.current_activity_hash)
            .await;

        match activity {
            Ok(a) => a,
            Err(ApiError::ResponseError(BungieResponseError::ResponseMissing)) => {
                *last_activity = None;
                return Ok(true);
            }
            Err(e) => return Err(e.into()),
        }
    };

    let current_activity = CurrentActivity {
        start_date: latest_activity.date_activity_started,
        activity_info: current_activity_info,
    };

    *last_activity = Some(current_activity);

    Ok(true)
}

async fn update_history(
    handle: &AppHandle,
    last_history: &mut Vec<CompletedActivity>,
    profile: &Profile,
) -> Result<bool> {
    let api = handle.state::<Api>();

    let profile_info = api.profile_info_source.lock().await.get(profile).await?;

    let mut past_activities: Vec<CompletedActivity> = Vec::new();

    let cutoff = {
        let mut time = Utc::today().and_hms(17, 0, 0); // 5PM UTC

        if time > Utc::now() {
            time -= chrono::Duration::days(1);
        }

        time
    };

    for character_id in profile_info.character_ids.iter() {
        let mut page = 0;

        loop {
            let history = Api::get_activity_history(profile, character_id, page).await?;

            let activities = match history.activities {
                Some(a) => a,
                None => break,
            };

            let mut includes_past_cutoff = false;

            for activity in activities.into_iter() {
                if activity.period < cutoff {
                    includes_past_cutoff = true;
                } else {
                    past_activities.push(activity);
                }
            }

            if includes_past_cutoff {
                break;
            }

            page += 1;
        }
    }

    if let Some(last) = last_history.into_iter().max() {
        if let Some(new) = (&mut past_activities).into_iter().max() {
            if last > new {
                // replace with >=, see current
                return Ok(false);
            }
        }
    }

    past_activities.sort();

    let sorted_activities = past_activities.into_iter().rev().collect();

    *last_history = sorted_activities;

    Ok(true)
}
