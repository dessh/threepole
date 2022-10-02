#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use anyhow::{anyhow, Error};
use api::{
    make_request,
    responses::{
        CharacterActivityHistory, CharacterCurrentActivities, CompletedActivity,
        DestinyActivityDefinition, DestinyMembership,
    },
    BungieRequest, BungieResponseError,
};
use chrono::{DateTime, Utc};
use config::Config;
use consts::{APP_NAME, POLL_INTERVAL, RAID_ACTIVITY_TYPE};
use poller::poll_focus;
use serde::Serialize;
use tauri::{
    async_runtime, AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, WindowBuilder, WindowUrl,
};
use tokio::sync::Mutex;

mod api;
mod config;
mod consts;
mod poller;

#[derive(Default)]
struct ConfigContainer(Mutex<Option<Config>>);

#[derive(Default)]
struct ActivityTypes(Mutex<HashMap<usize, usize>>);

#[derive(Default)]
struct CharacterIds(Mutex<Vec<String>>);

#[derive(Serialize)]
struct CurrentActivity {
    latest_activity_started: DateTime<Utc>,
    is_raid: bool,
}

#[derive(Serialize)]
struct ActivityHistory {
    total_today: usize,
    latest_activity_completed: Option<CompletedActivity>,
}

struct SerializableError(Error);

impl Serialize for SerializableError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct ErrorSerialization {
            message: String,
            backtrace: String,
        }

        ErrorSerialization {
            message: self.0.to_string(),
            backtrace: self.0.backtrace().to_string(),
        }
        .serialize(serializer)
    }
}

#[tauri::command]
async fn search_profile(
    display_name: String,
    display_name_code: usize,
) -> Result<Vec<DestinyMembership>, SerializableError> {
    let res = make_request(BungieRequest::SearchDestinyPlayerByBungieName {
        display_name: &display_name,
        display_name_code,
    })
    .await
    .map_err(|e| SerializableError(e.into()))?;

    Ok(serde_json::from_value(res).map_err(|e| SerializableError(e.into()))?)
}

#[tauri::command]
async fn get_config(container: State<'_, ConfigContainer>) -> Result<Option<Config>, ()> {
    Ok(container.0.lock().await.clone())
}

#[tauri::command]
async fn set_config(
    app: AppHandle,
    config: Config,
    container: State<'_, ConfigContainer>,
) -> Result<(), ()> {
    config.write().await.unwrap();

    let mut lock = container.0.lock().await;
    *lock = Some(config);

    if let Some(o) = app.get_window("overlay") {
        o.emit("force_refresh", ()).unwrap();
    } else {
        create_overlay(&app).unwrap();
    }

    Ok(())
}

#[tauri::command]
async fn get_current_activity(
    container: State<'_, ConfigContainer>,
    characters: State<'_, CharacterIds>,
    activity_types: State<'_, ActivityTypes>,
) -> Result<CurrentActivity, SerializableError> {
    let (membership_type, membership_id) = {
        let lock = container.0.lock().await;

        match lock.as_ref() {
            Some(c) => (c.account_platform, c.account_id.clone()),
            None => return Err(SerializableError(anyhow!("No profile set"))),
        }
    };

    let res = make_request(BungieRequest::GetProfile {
        membership_type,
        membership_id: &membership_id,
    })
    .await
    .map_err(|e| SerializableError(e.into()))?;

    let current_activities: CharacterCurrentActivities =
        serde_json::from_value(res).map_err(|e| SerializableError(e.into()))?;

    let activities = match current_activities.activities {
        Some(a) => a,
        None => return Err(SerializableError(anyhow!("Profile is private"))),
    };

    {
        let mut lock = characters.0.lock().await;
        lock.clear();
        for activity in activities.iter() {
            lock.push(activity.character_id.clone());
        }
    }

    let latest_activity = activities
        .iter()
        .max()
        .ok_or(SerializableError(anyhow!("No character data for profile")))?;

    let is_raid = {
        let mut lock = activity_types.0.lock().await;

        let activity_type = if let Some(v) = lock.get(&latest_activity.current_activity_hash) {
            *v
        } else {
            match make_request(BungieRequest::GetDestinyActivityDefinition {
                activity_hash: latest_activity.current_activity_hash,
            })
            .await
            {
                Ok(res) => {
                    let activity_definition: DestinyActivityDefinition =
                        serde_json::from_value(res).map_err(|e| SerializableError(e.into()))?;

                    lock.insert(
                        latest_activity.current_activity_hash,
                        activity_definition.activity_type_hash,
                    );

                    activity_definition.activity_type_hash
                }
                Err(BungieResponseError::ResponseMissing) => 0,
                Err(e) => return Err(SerializableError(e.into())),
            }
        };

        activity_type == RAID_ACTIVITY_TYPE
    };

    Ok(CurrentActivity {
        latest_activity_started: latest_activity.date_activity_started,
        is_raid,
    })
}

#[tauri::command]
async fn get_history(
    container: State<'_, ConfigContainer>,
    characters: State<'_, CharacterIds>,
) -> Result<ActivityHistory, SerializableError> {
    let (membership_type, membership_id) = {
        let lock = container.0.lock().await;

        match lock.as_ref() {
            Some(c) => (c.account_platform, c.account_id.clone()),
            None => return Err(SerializableError(anyhow!("No profile set"))),
        }
    };

    let characters = { characters.0.lock().await.clone() };

    let mut past_activities: HashSet<CompletedActivity> = HashSet::new();

    let cutoff = {
        let mut time = Utc::today().and_hms(17, 0, 0); // 5PM UTC

        if time > Utc::now() {
            time -= chrono::Duration::days(1);
        }

        time
    };

    for character_id in characters {
        let mut page = 0;

        loop {
            let res = make_request(BungieRequest::GetActivityHistory {
                membership_type,
                membership_id: &membership_id,
                character_id: &character_id,
                page,
            })
            .await
            .map_err(|e| SerializableError(e.into()))?;

            let history: CharacterActivityHistory =
                serde_json::from_value(res).map_err(|e| SerializableError(e.into()))?;

            let activities = match history.activities {
                Some(a) => a,
                None => break,
            };

            let mut includes_past_cutoff = false;

            for activity in activities.into_iter() {
                if activity.period < cutoff {
                    includes_past_cutoff = true;
                } else {
                    if activity.completed {
                        past_activities.insert(activity);
                    }
                }
            }

            if includes_past_cutoff {
                break;
            }

            page += 1;
        }
    }

    Ok(ActivityHistory {
        total_today: past_activities.len(),
        latest_activity_completed: past_activities.into_iter().max(),
    })
}

fn create_setup_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(handle, "setup", WindowUrl::App("setup.html".into()))
        .title(APP_NAME)
        .transparent(true)
        .decorations(false)
        .inner_size(400.0, 500.0)
        .resizable(false)
        .always_on_top(true)
        .visible(false)
        .build()?;

    Ok(())
}

fn create_overlay(handle: &AppHandle) -> Result<(), tauri::Error> {
    let overlay = WindowBuilder::new(handle, "overlay", WindowUrl::App("overlay.html".into()))
        .title(APP_NAME)
        .transparent(true)
        .decorations(false)
        .inner_size(400.0, 500.0)
        .resizable(false)
        .always_on_top(true)
        .inner_size(0.0, 0.0)
        .position(0.0, 0.0)
        .visible(false)
        .skip_taskbar(true)
        .build()?;

    overlay.set_ignore_cursor_events(true).unwrap();

    #[cfg(debug_assertions)]
    overlay.open_devtools();

    async_runtime::spawn(async move {
        let mut hwnd_names = HashMap::new();

        loop {
            poll_focus(&overlay, &mut hwnd_names);

            tokio::time::sleep(Duration::from_millis(POLL_INTERVAL)).await;
        }
    });

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tauri::Builder::new()
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("set_profile", "Set profile"))
                    .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("exit", "Exit")),
            ),
        )
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "exit" => app.exit(0),
                    "set_profile" => match app.get_window("setup") {
                        Some(w) => w.set_focus(),
                        None => create_setup_window(app),
                    }
                    .unwrap(),
                    _ => (),
                }
            }
        })
        .manage(ConfigContainer::default())
        .manage(ActivityTypes::default())
        .manage(CharacterIds::default())
        .invoke_handler(tauri::generate_handler![
            get_config,
            search_profile,
            set_config,
            get_current_activity,
            get_history,
        ])
        .setup(|app| {
            let handle = app.handle();
            async_runtime::spawn(async move {
                if let Ok(c) = Config::load().await {
                    let container = handle.state::<ConfigContainer>();
                    let mut lock = container.0.lock().await;
                    *lock = Some(c);
                    create_overlay(&handle).unwrap();
                } else {
                    create_setup_window(&handle).unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())?;

    Ok(())
}
