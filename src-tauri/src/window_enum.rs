use windows::Win32::Foundation::{BOOL, HWND, LPARAM, TRUE};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowLongW, GetWindowTextLengthW, GetWindowTextW, GetWindow,
    IsWindowVisible, GWL_EXSTYLE, GW_OWNER, WS_EX_TOOLWINDOW, WS_EX_NOACTIVATE,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows::core::PWSTR;

use crate::models::WindowInfo;

/// All windows across all desktops (includes cloaked/hidden)
pub fn enumerate_all_windows() -> Vec<WindowInfo> {
    let mut windows: Vec<WindowInfo> = Vec::new();
    unsafe {
        let _ = EnumWindows(
            Some(enum_all_callback),
            LPARAM(&mut windows as *mut Vec<WindowInfo> as isize),
        );
    }
    windows
}

/// Only visible windows on the current desktop
pub fn enumerate_windows() -> Vec<WindowInfo> {
    let mut windows: Vec<WindowInfo> = Vec::new();
    unsafe {
        let _ = EnumWindows(
            Some(enum_callback),
            LPARAM(&mut windows as *mut Vec<WindowInfo> as isize),
        );
    }
    windows
}

unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // Skip invisible windows
    if !IsWindowVisible(hwnd).as_bool() {
        return TRUE;
    }

    // Skip cloaked windows (hidden UWP apps, Store frames, etc.)
    let mut cloaked: u32 = 0;
    let _ = DwmGetWindowAttribute(
        hwnd,
        DWMWA_CLOAKED,
        &mut cloaked as *mut u32 as *mut _,
        std::mem::size_of::<u32>() as u32,
    );
    if cloaked != 0 {
        return TRUE;
    }

    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;

    // Skip tool windows
    if ex_style & WS_EX_TOOLWINDOW.0 != 0 {
        return TRUE;
    }

    // Skip noactivate windows (system overlays, tooltips)
    if ex_style & WS_EX_NOACTIVATE.0 != 0 {
        return TRUE;
    }

    // Skip windows that have an owner (popups, dialogs) — they follow their parent
    if GetWindow(hwnd, GW_OWNER).is_ok() {
        return TRUE;
    }

    // Get window title
    let title_len = GetWindowTextLengthW(hwnd);
    if title_len == 0 {
        return TRUE;
    }

    let mut title_buf = vec![0u16; (title_len + 1) as usize];
    GetWindowTextW(hwnd, &mut title_buf);
    let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

    // Get process exe name
    let exe_name = get_process_exe_name(hwnd).unwrap_or_default();

    // Get the virtual desktop this window belongs to
    let desktop_id = winvd::get_desktop_by_window(hwnd)
        .ok()
        .and_then(|d| d.get_id().ok())
        .map(|id| format!("{:?}", id));

    let window_info = WindowInfo {
        handle: hwnd.0 as isize,
        title,
        exe_name,
        desktop_id,
        is_visible: true,
    };

    let windows = &mut *(lparam.0 as *mut Vec<WindowInfo>);
    windows.push(window_info);

    TRUE
}

unsafe extern "system" fn enum_all_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;

    if ex_style & WS_EX_TOOLWINDOW.0 != 0 {
        return TRUE;
    }
    if ex_style & WS_EX_NOACTIVATE.0 != 0 {
        return TRUE;
    }
    if GetWindow(hwnd, GW_OWNER).is_ok() {
        return TRUE;
    }

    let title_len = GetWindowTextLengthW(hwnd);
    if title_len == 0 {
        return TRUE;
    }

    let mut title_buf = vec![0u16; (title_len + 1) as usize];
    GetWindowTextW(hwnd, &mut title_buf);
    let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

    let exe_name = get_process_exe_name(hwnd).unwrap_or_default();

    let desktop_id = winvd::get_desktop_by_window(hwnd)
        .ok()
        .and_then(|d| d.get_id().ok())
        .map(|id| format!("{:?}", id));

    let is_visible = IsWindowVisible(hwnd).as_bool();

    let window_info = WindowInfo {
        handle: hwnd.0 as isize,
        title,
        exe_name,
        desktop_id,
        is_visible,
    };

    let windows = &mut *(lparam.0 as *mut Vec<WindowInfo>);
    windows.push(window_info);

    TRUE
}

unsafe fn get_process_exe_name(hwnd: HWND) -> Option<String> {
    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));
    if process_id == 0 {
        return None;
    }

    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id).ok()?;
    let mut buf = vec![0u16; 260];
    let mut size = buf.len() as u32;
    QueryFullProcessImageNameW(handle, PROCESS_NAME_FORMAT(0), PWSTR(buf.as_mut_ptr()), &mut size).ok()?;

    let full_path = String::from_utf16_lossy(&buf[..size as usize]);
    let exe_name = full_path
        .rsplit('\\')
        .next()
        .unwrap_or(&full_path)
        .to_string();
    Some(exe_name)
}
