use tauri::State;

use crate::desktop_manager;
use crate::models::{WindowAssignment, WindowInfo};
use crate::state::AppStateMutex;
use crate::window_enum;

use super::projects::save_state;

#[tauri::command]
pub fn list_open_windows() -> Result<Vec<WindowInfo>, String> {
    Ok(window_enum::enumerate_windows())
}

#[tauri::command]
pub fn list_all_windows() -> Result<Vec<WindowInfo>, String> {
    Ok(window_enum::enumerate_all_windows())
}

#[tauri::command]
pub fn assign_window_to_project(
    state: State<'_, AppStateMutex>,
    project_id: String,
    window_handle: isize,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let project = state
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or("Project not found")?
        .clone();

    let desktop_guid = project
        .desktop_guid
        .as_ref()
        .ok_or("Project has no desktop")?;

    // Find target desktop index
    let desktops = desktop_manager::list_desktops()?;
    let desktop = desktops
        .iter()
        .find(|d| &d.guid == desktop_guid)
        .ok_or("Desktop not found")?;

    // Move window to the project's desktop
    desktop_manager::move_window_to_desktop(window_handle, desktop.index)?;

    // Get window info for the assignment record
    let all_windows = window_enum::enumerate_windows();
    let window = all_windows.iter().find(|w| w.handle == window_handle);

    let assignment = WindowAssignment {
        project_id: project_id.clone(),
        window_handle,
        window_title: window.map(|w| w.title.clone()).unwrap_or_default(),
        exe_name: window.map(|w| w.exe_name.clone()).unwrap_or_default(),
        assigned_at: chrono::Utc::now().to_rfc3339(),
    };

    // Remove any existing assignment for this window
    state.assignments.retain(|a| a.window_handle != window_handle);
    state.assignments.push(assignment);

    save_state(&state)?;
    Ok(())
}

#[tauri::command]
pub fn unassign_window(
    state: State<'_, AppStateMutex>,
    window_handle: isize,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.assignments.retain(|a| a.window_handle != window_handle);
    save_state(&state)?;
    Ok(())
}

#[tauri::command]
pub fn kill_window_process(
    state: State<'_, AppStateMutex>,
    window_handle: isize,
) -> Result<(), String> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
    use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

    unsafe {
        let hwnd = HWND(window_handle as *mut _);
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return Err("Could not get process ID".into());
        }

        let handle = OpenProcess(PROCESS_TERMINATE, false, pid)
            .map_err(|e| format!("Could not open process: {e}"))?;
        TerminateProcess(handle, 1)
            .map_err(|e| format!("Could not terminate process: {e}"))?;
    }

    // Remove assignment
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.assignments.retain(|a| a.window_handle != window_handle);
    save_state(&state)?;

    Ok(())
}

#[derive(serde::Serialize, Clone)]
pub struct OtherWindowInfo {
    pub window_handle: isize,
    pub window_title: String,
    pub exe_name: String,
    pub project_id: String,
    pub project_name: String,
}

#[tauri::command]
pub fn get_other_project_windows(
    state: State<'_, AppStateMutex>,
    exclude_project_id: String,
) -> Result<Vec<OtherWindowInfo>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state
        .assignments
        .iter()
        .filter(|a| a.project_id != exclude_project_id)
        .map(|a| {
            let project_name = state
                .projects
                .iter()
                .find(|p| p.id == a.project_id)
                .map(|p| p.name.clone())
                .unwrap_or_default();
            OtherWindowInfo {
                window_handle: a.window_handle,
                window_title: a.window_title.clone(),
                exe_name: a.exe_name.clone(),
                project_id: a.project_id.clone(),
                project_name,
            }
        })
        .collect())
}

#[tauri::command]
pub fn get_project_windows(
    state: State<'_, AppStateMutex>,
    project_id: String,
) -> Result<Vec<WindowAssignment>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state
        .assignments
        .iter()
        .filter(|a| a.project_id == project_id)
        .cloned()
        .collect())
}
