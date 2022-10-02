use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{collections::HashMap, path::PathBuf};
use tauri::{PhysicalPosition, PhysicalSize};
use widestring::Utf16String;
use windows::Win32::{
    Foundation::{HWND, MAX_PATH, RECT},
    System::{
        ProcessStatus::K32GetModuleFileNameExW,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
    },
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId},
};

use crate::consts::TARGET_NAME;

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

pub fn poll_focus(window: &tauri::Window, hwnd_names: &mut HashMap<isize, String>) {
    let hwnd = unsafe { GetForegroundWindow() };

    if let RawWindowHandle::Win32(h) = window.raw_window_handle() {
        if h.hwnd as isize == hwnd.0 {
            return;
        }
    }

    let focused_name = if let Some(n) = hwnd_names.get(&hwnd.0) {
        n
    } else {
        match get_hwnd_exec(hwnd) {
            Some(n) => {
                hwnd_names.insert(hwnd.0, n);
                hwnd_names.get(&hwnd.0).unwrap()
            }
            None => return,
        }
    };

    if focused_name != TARGET_NAME {
        window.emit("hide", ()).unwrap();
        return;
    }

    let mut dims = RECT::default();

    unsafe { GetWindowRect(hwnd, &mut dims) };

    window
        .set_position(PhysicalPosition {
            x: dims.left,
            y: dims.top,
        })
        .unwrap();

    window
        .set_size(PhysicalSize {
            width: dims.right - dims.left,
            height: dims.bottom - dims.top,
        })
        .unwrap();

    window.emit("show", ()).unwrap();
}
