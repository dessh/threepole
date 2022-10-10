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
        responses::{ActivityInfo, CompletedActivity, LatestCharacterActivity, ProfileInfo},
        Api, ApiError, Source,
    },
    config::profiles::Profile,
    consts::API_POLL_INTERVAL,
    ConfigContainer,
};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    current_activity: CurrentActivity,
    activity_history: Vec<CompletedActivity>,
    profile_info: ProfileInfo,
}

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDataStatus {
    last_update: Option<PlayerData>,
    error: Option<String>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CurrentActivity {
    start_date: DateTime<Utc>,
    activity_hash: usize,
    activity_info: Option<ActivityInfo>,
}

#[derive(Default)]
pub struct PlayerDataPoller {
    task_handle: Option<JoinHandle<()>>,
    current_playerdata: Arc<Mutex<PlayerDataStatus>>,
    // TODO: maybe add preferences/profiles the poller was started on to see if it needs a reset on change
    // think about reset ting
}

impl PlayerDataPoller {
    pub async fn reset(&mut self, app_handle: AppHandle) {
        if let Some(t) = self.task_handle.as_ref() {
            t.abort();
        }

        {
            let mut lock = self.current_playerdata.lock().await;
            *lock = PlayerDataStatus::default();

            send_data_update(&app_handle, lock.clone());
        }

        let playerdata_clone = self.current_playerdata.clone();

        self.task_handle = Some(async_runtime::spawn(async move {
            let profile = {
                let container = app_handle.state::<ConfigContainer>();
                let lock = container.0.lock().await;

                match &lock.get_profiles().selected_profile {
                    Some(p) => p.clone(),
                    None => {
                        let mut lock = playerdata_clone.lock().await;
                        lock.error = Some("No profile set".to_string());

                        send_data_update(&app_handle, lock.clone());
                        return;
                    }
                }
            };

            let profile_info = {
                let api = app_handle.state::<Api>();
                let mut lock = api.profile_info_source.lock().await;

                match lock.get(&profile).await {
                    Ok(p) => p,
                    Err(e) => {
                        let mut lock = playerdata_clone.lock().await;
                        lock.error = Some(format!("Failed to get profile info: {e}"));

                        send_data_update(&app_handle, lock.clone());
                        return;
                    }
                }
            };

            let mut current_activity = CurrentActivity {
                start_date: DateTime::<Utc>::MIN_UTC,
                activity_hash: 0,
                activity_info: None,
            };
            let mut activity_history = Vec::new();

            let res = match update_current(&app_handle, &mut current_activity, &profile).await {
                Ok(_) => update_history(&app_handle, &mut activity_history, &profile).await,
                Err(e) => Err(e),
            };

            {
                let mut lock = playerdata_clone.lock().await;
                match res {
                    Ok(_) => {
                        let playerdata = PlayerData {
                            current_activity: current_activity,
                            activity_history,
                            profile_info,
                        };

                        lock.last_update = Some(playerdata);
                    }
                    Err(e) => {
                        lock.error = Some(e.to_string());
                    }
                }

                send_data_update(&app_handle, lock.clone());
            }

            let mut count = 0;

            loop {
                tokio::time::sleep(API_POLL_INTERVAL).await;

                let mut last_update = playerdata_clone.lock().await.last_update.clone().unwrap();

                let res = if count < 5 {
                    update_current(&app_handle, &mut last_update.current_activity, &profile).await
                } else {
                    count = 0;
                    update_history(&app_handle, &mut last_update.activity_history, &profile).await
                };

                // The boolean return value of update_* functions represents whether or not
                // the last_update should be resent to the overlay / details

                match res {
                    Ok(true) => {
                        let mut lock = playerdata_clone.lock().await;
                        lock.error = None;
                        lock.last_update = Some(last_update);

                        send_data_update(&app_handle, lock.clone())
                    }
                    Err(e) => {
                        let mut lock = playerdata_clone.lock().await;
                        lock.error = Some(e.to_string());

                        send_data_update(&app_handle, lock.clone())
                    }
                    _ => (),
                }

                count += 1;
            }
        }));
    }

    // For overlay / detail window to get initial data instead of waiting for poll
    pub fn get_data(&mut self) -> Option<PlayerDataStatus> {
        return match &self.current_playerdata.try_lock() {
            Ok(p) => Some((*p).clone()), // If playerdata doesn't exist, meaning poller isn't initialized
            Err(_) => None, // If lock currently in use, meaning stat update is in progress
        };
    }
}

fn send_data_update(handle: &AppHandle, data: PlayerDataStatus) {
    if let Some(o) = handle.get_window("overlay") {
        o.emit("playerdata_update", data.clone()).unwrap();
    }

    if let Some(o) = handle.get_window("details") {
        o.emit("playerdata_update", data).unwrap();
    }
}

async fn update_current(
    handle: &AppHandle,
    last_activity: &mut CurrentActivity,
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

    match last_activity
        .start_date
        .cmp(&latest_activity.date_activity_started)
    {
        std::cmp::Ordering::Less => {
            last_activity.start_date = latest_activity.date_activity_started
        }
        std::cmp::Ordering::Equal => {
            if last_activity.activity_info.is_none() {
                return Ok(false);
                // Return here, as once activity_info becomes None
                // for a given activity start_date, it should
                // stay None until start_date changes again
            }

            if last_activity.activity_hash == latest_activity.current_activity_hash {
                return Ok(false);
                // Return if the activity hash and time are the same
            }
        }
        std::cmp::Ordering::Greater => return Ok(false),
        // Only return if our last-fetched activity is more recent,
        // as current_hash can change without start_date changing
    }

    let api = handle.state::<Api>();

    api.profile_info_source
        .lock()
        .await
        .set_characters(profile, characters);

    if latest_activity.current_activity_hash == 0 {
        last_activity.activity_info = None;
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
                last_activity.activity_info = None;
                return Ok(true);
            }
            Err(e) => return Err(e.into()),
        }
    };

    if current_activity_info.name.is_empty() {
        last_activity.activity_info = None;
        return Ok(true);
    }

    last_activity.activity_hash = latest_activity.current_activity_hash;
    last_activity.activity_info = Some(current_activity_info);

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
            if last >= new {
                return Ok(false);
            }
        }
    }

    past_activities.sort();

    let sorted_activities = past_activities.into_iter().rev().collect();

    *last_history = sorted_activities;

    Ok(true)
}
