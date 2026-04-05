use crate::models::DesktopInfo;
use windows::Win32::Foundation::HWND;
use winvd::{get_current_desktop, get_desktops, switch_desktop};

fn err(msg: &str, e: winvd::Error) -> String {
    format!("{msg}: {e:?}")
}

pub fn list_desktops() -> Result<Vec<DesktopInfo>, String> {
    let desktops = get_desktops().map_err(|e| err("Failed to get desktops", e))?;
    let current = get_current_desktop().map_err(|e| err("Failed to get current", e))?;
    let current_guid = format!("{:?}", current.get_id().map_err(|e| err("get_id", e))?);

    let mut result = Vec::new();
    for (index, desktop) in desktops.iter().enumerate() {
        let guid = format!("{:?}", desktop.get_id().map_err(|e| err("get_id", e))?);
        let name = desktop
            .get_name()
            .ok()
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| format!("Desktop {}", index + 1));

        result.push(DesktopInfo {
            guid: guid.clone(),
            name,
            index: index as u32,
            is_current: guid == current_guid,
        });
    }

    Ok(result)
}

pub fn get_current() -> Result<DesktopInfo, String> {
    let current = get_current_desktop().map_err(|e| err("Failed to get current", e))?;
    let guid = format!("{:?}", current.get_id().map_err(|e| err("get_id", e))?);
    let index = current.get_index().map_err(|e| err("get_index", e))?;
    let name = current
        .get_name()
        .ok()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| format!("Desktop {}", index + 1));

    Ok(DesktopInfo {
        guid,
        name,
        index,
        is_current: true,
    })
}

pub fn create_desktop() -> Result<DesktopInfo, String> {
    let desktop = winvd::create_desktop().map_err(|e| err("Failed to create desktop", e))?;
    let guid = format!("{:?}", desktop.get_id().map_err(|e| err("get_id", e))?);
    let index = desktop.get_index().map_err(|e| err("get_index", e))?;
    let name = desktop
        .get_name()
        .ok()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| format!("Desktop {}", index + 1));

    Ok(DesktopInfo {
        guid,
        name,
        index,
        is_current: false,
    })
}

pub fn switch_to_desktop(index: u32) -> Result<(), String> {
    let desktops = get_desktops().map_err(|e| err("Failed to get desktops", e))?;
    let desktop = desktops
        .get(index as usize)
        .ok_or_else(|| format!("Desktop index {index} out of range"))?;
    switch_desktop(desktop.get_index().map_err(|e| err("get_index", e))?)
        .map_err(|e| err("Failed to switch", e))
}

pub fn remove_desktop(index: u32) -> Result<(), String> {
    let desktops = get_desktops().map_err(|e| err("Failed to get desktops", e))?;
    if index as usize >= desktops.len() {
        return Err(format!("Desktop index {index} out of range"));
    }
    let target_idx = desktops[index as usize]
        .get_index()
        .map_err(|e| err("get_index", e))?;
    let fallback_idx = desktops[0]
        .get_index()
        .map_err(|e| err("get_index", e))?;
    winvd::remove_desktop(target_idx, fallback_idx).map_err(|e| err("Failed to remove", e))
}

pub fn rename_desktop(index: u32, name: &str) -> Result<(), String> {
    let desktops = get_desktops().map_err(|e| err("Failed to get desktops", e))?;
    let desktop = desktops
        .get(index as usize)
        .ok_or_else(|| format!("Desktop index {index} out of range"))?;
    desktop.set_name(name).map_err(|e| err("Failed to rename", e))
}

pub fn move_window_to_desktop(hwnd: isize, desktop_index: u32) -> Result<(), String> {
    let desktops = get_desktops().map_err(|e| err("Failed to get desktops", e))?;
    if desktop_index as usize >= desktops.len() {
        return Err(format!("Desktop index {desktop_index} out of range"));
    }
    let desk_idx = desktops[desktop_index as usize]
        .get_index()
        .map_err(|e| err("get_index", e))?;

    let hwnd = HWND(hwnd as *mut _);
    winvd::move_window_to_desktop(desk_idx, &hwnd)
        .map_err(|e| err("Failed to move window", e))
}
