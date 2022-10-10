#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use api::{
    responses::{ActivityInfo, BungieProfile, ProfileInfo},
    Api, Source,
};
use config::{
    preferences::Preferences,
    profiles::{Profile, Profiles},
    ConfigManager,
};
use consts::{APP_NAME, APP_VER};
use pollers::{
    overlay::overlay_poller,
    playerdata::{PlayerDataPoller, PlayerDataStatus},
};
use tauri::{
    async_runtime::{self, JoinHandle},
    AppHandle, CustomMenuItem, Manager, RunEvent, State, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, WindowBuilder, WindowUrl,
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

// https://github.com/tauri-apps/wry/issues/583
fn open_preferences_sync(handle: AppHandle) -> Result<(), tauri::Error> {
    match handle.get_window("preferences") {
        Some(w) => w.set_focus(),
        None => create_preferences_window(&handle),
    }
}

// https://github.com/tauri-apps/wry/issues/583
fn open_profiles_sync(handle: AppHandle) -> Result<(), tauri::Error> {
    match handle.get_window("profiles") {
        Some(w) => w.set_focus(),
        None => create_profiles_window(&handle),
    }
}

#[tauri::command]
async fn open_preferences(handle: AppHandle) -> Result<(), tauri::Error> {
    open_preferences_sync(handle)
}

#[tauri::command]
async fn open_profiles(handle: AppHandle) -> Result<(), tauri::Error> {
    open_profiles_sync(handle)
}

#[tauri::command]
async fn get_preferences(container: State<'_, ConfigContainer>) -> Result<Preferences, ()> {
    Ok(container.0.lock().await.get_preferences().clone())
}

#[tauri::command]
async fn set_preferences(
    handle: AppHandle,
    preferences: Preferences,
    container: State<'_, ConfigContainer>,
    poller_handle: State<'_, OverlayPollerHandle>,
) -> Result<(), ()> {
    let mut lock = container.0.lock().await;
    lock.set_preferences(preferences.clone()).unwrap();

    if let Some(o) = handle.get_window("overlay") {
        if preferences.enable_overlay {
            o.emit("preferences_update", preferences).unwrap();
        } else {
            if let Some(h) = poller_handle.0.lock().await.as_ref() {
                h.abort();
            }

            o.close().unwrap();
        }
    } else if preferences.enable_overlay {
        create_overlay(handle).await.unwrap();
    }

    Ok(())
}

#[tauri::command]
async fn get_profiles(container: State<'_, ConfigContainer>) -> Result<Profiles, ()> {
    Ok(container.0.lock().await.get_profiles().clone())
}

#[tauri::command]
async fn set_profiles(
    handle: AppHandle,
    profiles: Profiles,
    config_container: State<'_, ConfigContainer>,
    poller_container: State<'_, PlayerDataPollerContainer>,
) -> Result<(), ()> {
    let mut lock = config_container.0.lock().await;
    lock.set_profiles(profiles).unwrap();

    poller_container.0.lock().await.reset(handle).await;

    Ok(())
}

#[tauri::command]
async fn get_profile_info(profile: Profile, api: State<'_, Api>) -> Result<ProfileInfo, String> {
    Ok(api
        .profile_info_source
        .lock()
        .await
        .get(&profile)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn get_activity_info(
    activity_hash: usize,
    api: State<'_, Api>,
) -> Result<ActivityInfo, String> {
    Ok(api
        .activity_info_source
        .lock()
        .await
        .get(&activity_hash)
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
        WindowUrl::App("./src/overlay/overlay.html".into()),
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
) -> Result<Option<PlayerDataStatus>, ()> {
    Ok(poller_container.0.lock().await.get_data())
}

fn create_preferences_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(
        handle,
        "preferences",
        WindowUrl::App("./src/window/window.html#preferences".into()),
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

fn create_profiles_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(
        handle,
        "profiles",
        WindowUrl::App("./src/window/window.html#profiles".into()),
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

fn create_details_window(handle: &AppHandle) -> Result<(), tauri::Error> {
    WindowBuilder::new(
        handle,
        "details",
        WindowUrl::App("./src/window/window.html#details".into()),
    )
    .title(APP_NAME)
    .transparent(true)
    .decorations(false)
    .inner_size(600.0, 600.0)
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
                    .add_item(
                        CustomMenuItem::new("version_info", format!("{APP_NAME} v{}", APP_VER))
                            .disabled(),
                    )
                    .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("preferences", "Preferences"))
                    .add_item(CustomMenuItem::new("set_profile", "Set profile"))
                    .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("exit", "Exit")),
            ),
        )
        .on_system_tray_event(|handle, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "exit" => handle.exit(0),
                    "set_profile" => open_profiles_sync(handle.clone()).unwrap(),
                    "preferences" => open_preferences_sync(handle.clone()).unwrap(),
                    _ => (),
                }
            } else if let SystemTrayEvent::LeftClick { .. } = event {
                match handle.get_window("details") {
                    Some(w) => w.set_focus(),
                    None => create_details_window(handle),
                }
                .unwrap()
            }
        })
        .invoke_handler(tauri::generate_handler![
            open_preferences,
            open_profiles,
            get_preferences,
            set_preferences,
            get_profiles,
            set_profiles,
            get_profile_info,
            get_activity_info,
            search_profile,
            get_playerdata,
        ])
        .setup(|app| {
            let handle = app.handle();

            async_runtime::spawn(async move {
                let config_container = handle.state::<ConfigContainer>();
                let poller_container = handle.state::<PlayerDataPollerContainer>();

                poller_container.0.lock().await.reset(handle.clone()).await;

                let lock = config_container.0.lock().await;

                if lock.get_profiles().selected_profile.is_none() {
                    create_profiles_window(&handle).unwrap();
                }

                if lock.get_preferences().enable_overlay {
                    create_overlay(handle.clone()).await.unwrap();
                }

                create_details_window(&handle).unwrap();
            });

            Ok(())
        })
        .build(tauri::generate_context!())?
        .run(|_, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => (),
        });

    Ok(())
}
