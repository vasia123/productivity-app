use tauri::State;

use crate::desktop_manager;
use crate::models::{PersistenceData, WindowAssignment, WindowInfo};
use crate::persistence;
use crate::state::AppStateMutex;
use crate::window_enum;

#[tauri::command]
pub fn list_open_windows() -> Result<Vec<WindowInfo>, String> {
    Ok(window_enum::enumerate_windows())
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

    let data = PersistenceData {
        projects: state.projects.clone(),
        assignments: state.assignments.clone(),
    };
    persistence::save_data(&state.data_path, &data)?;

    Ok(())
}

#[tauri::command]
pub fn unassign_window(
    state: State<'_, AppStateMutex>,
    window_handle: isize,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.assignments.retain(|a| a.window_handle != window_handle);

    let data = PersistenceData {
        projects: state.projects.clone(),
        assignments: state.assignments.clone(),
    };
    persistence::save_data(&state.data_path, &data)?;

    Ok(())
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
