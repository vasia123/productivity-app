use crate::desktop_manager;
use crate::models::DesktopInfo;

#[tauri::command]
pub fn list_desktops() -> Result<Vec<DesktopInfo>, String> {
    desktop_manager::list_desktops()
}

#[tauri::command]
pub fn get_current_desktop() -> Result<DesktopInfo, String> {
    desktop_manager::get_current()
}
