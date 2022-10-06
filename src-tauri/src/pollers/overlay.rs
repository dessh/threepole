use std::{collections::HashMap, path::PathBuf};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};
use widestring::Utf16String;
use windows::Win32::{
    Foundation::{HWND, MAX_PATH, RECT},
    System::{
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
    },
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId},
};

use crate::consts::{OVERLAY_POLL_INTERVAL, TARGET_NAME};

fn get_hwnd_exec(hwnd: HWND) -> Option<String> {
    if hwnd.0 == 0 {
        return None;
    }

    let mut process_id = 0;

    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };

    if process_id == 0 {
        return None;
    }

    let handle_res = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, process_id) };

    let h = match handle_res {
        Ok(h) => h,
        Err(_) => return None,
    };

    let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];

    unsafe { K32GetModuleFileNameExW(h, None, &mut buf) };

    let mut path_string = Utf16String::from_slice_lossy(&buf).to_string();
    path_string.retain(|c| c != '\0');

    let path = PathBuf::from(path_string);

    return path.file_name().map(|s| s.to_string_lossy().into_owned());
}

async fn poll(handle: &AppHandle, hwnd_names: &mut HashMap<isize, String>) {
    let overlay = match handle.get_window("overlay") {
        Some(h) => h,
        None => return,
    };

    let foreground_hwnd = unsafe { GetForegroundWindow() };

    if let RawWindowHandle::Win32(h) = overlay.raw_window_handle() {
        if h.hwnd as isize == foreground_hwnd.0 {
            return;
        }
    }

    let focused_name = if let Some(n) = hwnd_names.get(&foreground_hwnd.0) {
        n
    } else {
        match get_hwnd_exec(foreground_hwnd) {
            Some(n) => {
                hwnd_names.insert(foreground_hwnd.0, n);
                hwnd_names.get(&foreground_hwnd.0).unwrap()
            }
            None => return,
        }
    };

    if focused_name != TARGET_NAME {
        overlay.emit("hide", ()).unwrap();
        return;
    }

    let mut dims = RECT::default();

    unsafe { GetWindowRect(foreground_hwnd, &mut dims) };

    overlay
        .set_position(PhysicalPosition {
            x: dims.left,
            y: dims.top,
        })
        .unwrap();

    overlay
        .set_size(PhysicalSize {
            width: dims.right - dims.left,
            height: dims.bottom - dims.top,
        })
        .unwrap();

    overlay.emit("show", ()).unwrap();
}

pub async fn overlay_poller(handle: AppHandle) {
    let mut hwnd_names = HashMap::new();

    loop {
        poll(&handle, &mut hwnd_names).await;

        tokio::time::sleep(OVERLAY_POLL_INTERVAL).await;
    }
}
