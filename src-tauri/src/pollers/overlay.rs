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
    UI::{
        Shell::SHQueryUserNotificationState,
        WindowsAndMessaging::{GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId},
    },
};

use crate::consts::{OVERLAY_POLL_INTERVAL, TARGET_NAME};

enum PollResult {
    Open(HWND),
    Closed,
    Retain,
}

#[derive(Default)]
struct Poller {
    hwnd_names: HashMap<isize, String>,
}

impl Poller {
    async fn poll(&mut self, overlay_hwnd: isize) -> PollResult {
        let notification_state = unsafe { SHQueryUserNotificationState() };

        match notification_state {
            Ok(n) if n.0 == 3 => return PollResult::Closed, // If in DX exclusive fullscreen mode
            _ => (),
        }

        let foreground_hwnd = unsafe { GetForegroundWindow() };

        if overlay_hwnd == foreground_hwnd.0 {
            return PollResult::Retain;
        }

        let focused_name = match self.hwnd_names.get(&foreground_hwnd.0) {
            Some(n) => n,
            None => match get_hwnd_exec(foreground_hwnd) {
                Some(n) => {
                    self.hwnd_names.insert(foreground_hwnd.0, n);
                    self.hwnd_names.get(&foreground_hwnd.0).unwrap()
                }
                None => return PollResult::Closed,
            },
        };

        if focused_name == TARGET_NAME {
            PollResult::Open(foreground_hwnd)
        } else {
            PollResult::Closed
        }
    }
}

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

pub async fn overlay_poller(handle: AppHandle) {
    let mut poller = Poller::default();

    loop {
        let (overlay, overlay_hwnd) = {
            let overlay = match handle.get_window("overlay") {
                Some(h) => h,
                None => return,
            };

            match overlay.raw_window_handle() {
                RawWindowHandle::Win32(h) => {
                    let hwnd = h.hwnd as isize;
                    if hwnd == 0 {
                        return;
                    }

                    (overlay, hwnd)
                }
                _ => return,
            }
        };

        match poller.poll(overlay_hwnd).await {
            PollResult::Open(hwnd) => {
                let mut dims = RECT::default();

                unsafe { GetWindowRect(hwnd, &mut dims) };

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
            PollResult::Closed => overlay.emit("hide", ()).unwrap(),
            PollResult::Retain => (),
        }

        tokio::time::sleep(OVERLAY_POLL_INTERVAL).await;
    }
}
