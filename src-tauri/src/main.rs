#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use api::{
    responses::{BungieProfile, ProfileInfo},
    Api, Source,
};
use config::{
    preferences::Preferences,
    profiles::{Profile, Profiles},
    ConfigManager,
};
use consts::APP_NAME;
use pollers::{
    overlay::overlay_poller,
    playerdata::{PlayerData, PlayerDataPoller},
};
use tauri::{
    async_runtime::{self, JoinHandle},
    AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowBuilder, WindowUrl,
};
use tokio::sync::Mutex;

mod api;
mod config;
mod consts;
mod pollers;

struct ConfigContainer(Mutex<ConfigManager>);

#[derive(Default)]
struct PlayerDataPollerContainer(Mutex<PlayerDataPoller>);

#[derive(Default)]
struct OverlayPollerHandle(Mutex<Option<JoinHandle<()>>>);

#[tauri::command]
async fn get_preferences(container: State<'_, ConfigContainer>) -> Result<Preferences, ()> {
    Ok(container.0.lock().await.get_prefs().clone())
}

#[tauri::command]
async fn set_preferences(
    app: AppHandle,
    preferences: Preferences,
    container: State<'_, ConfigContainer>,
) -> Result<(), ()> {
    let mut lock = container.0.lock().await;
    lock.set_preferences(preferences.clone()).unwrap();

    if let Some(o) = app.get_window("overlay") {
        o.emit("preferences_update", preferences).unwrap();
    }

    Ok(())
}

#[tauri::command]
async fn get_profiles(container: State<'_, ConfigContainer>) -> Result<Profiles, ()> {
    Ok(container.0.lock().await.get_profiles().clone())
}

#[tauri::command]
async fn set_profiles(
    app: AppHandle,
    profiles: Profiles,
    config_container: State<'_, ConfigContainer>,
    poller_container: State<'_, PlayerDataPollerContainer>,
) -> Result<(), ()> {
    let mut lock = config_container.0.lock().await;
    lock.set_profiles(profiles).unwrap();

    poller_container.0.lock().await.reset(app);

    Ok(())
}

#[tauri::command]
async fn get_profile_info(profile: Profile, api: State<'_, Api>) -> Result<ProfileInfo, String> {
    Ok(api
        .profile_info_source
        .lock()
        .await
        .get(profile)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn search_profile(
    display_name: String,
    display_name_code: usize,
) -> Result<Vec<BungieProfile>, String> {
    Ok(Api::search_profile(&display_name, display_name_code)
        .await
        .map_err(|e| e.to_string())?)
}

async fn create_overlay(handle: AppHandle) -> Result<(), tauri::Error> {
    let overlay = WindowBuilder::new(
        &handle,
        "overlay",
        WindowUrl::App("./src/overlay.html".into()),
    )
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

    let handle_clone = handle.clone();
    let poller_handle = handle.state::<OverlayPollerHandle>();
    let mut lock = poller_handle.0.lock().await;

    if let Some(h) = lock.as_ref() {
        h.abort();
    }

    let handle = async_runtime::spawn(async move { overlay_poller(handle_clone).await });

    *lock = Some(handle);

    Ok(())
}

#[tauri::command]
async fn get_playerdata(
    poller_container: State<'_, PlayerDataPollerContainer>,
) -> Result<Option<PlayerData>, ()> {
    Ok(poller_container.0.lock().await.get_data())
}

fn create_profile_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(
        handle,
        "profile",
        WindowUrl::App("./src/profile.html".into()),
    )
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

fn create_preferences_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(
        handle,
        "preferences",
        WindowUrl::App("./src/preferences.html".into()),
    )
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

fn main() -> anyhow::Result<()> {
    tauri::Builder::new()
        .manage(ConfigContainer(Mutex::new(ConfigManager::load()?)))
        .manage(Api::default())
        .manage(PlayerDataPollerContainer::default())
        .manage(OverlayPollerHandle::default())
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("preferences", "Preferences"))
                    .add_item(CustomMenuItem::new("set_profile", "Set profile"))
                    .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("exit", "Exit")),
            ),
        )
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "exit" => app.exit(0),
                    "set_profile" => match app.get_window("profile") {
                        Some(w) => w.set_focus(),
                        None => create_profile_window(app),
                    }
                    .unwrap(),
                    "preferences" => match app.get_window("preferences") {
                        Some(w) => w.set_focus(),
                        None => create_preferences_window(app),
                    }
                    .unwrap(),
                    _ => (),
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_preferences,
            set_preferences,
            get_profiles,
            set_profiles,
            get_profile_info,
            search_profile,
            get_playerdata,
        ])
        .setup(|app| {
            let handle = app.handle();

            async_runtime::spawn(async move {
                let config_container = handle.state::<ConfigContainer>();
                let poller_container = handle.state::<PlayerDataPollerContainer>();

                create_overlay(handle.clone()).await.unwrap();

                poller_container.0.lock().await.reset(handle.clone());

                let lock = config_container.0.lock().await;

                if lock.get_profiles().selected_profile.is_none() {
                    create_profile_window(&handle).unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())?;

    Ok(())
}
