use tauri::State;

use crate::desktop_manager;
use crate::models::{PersistenceData, Project};
use crate::persistence;
use crate::state::AppStateMutex;

#[tauri::command]
pub fn list_projects(state: State<'_, AppStateMutex>) -> Result<Vec<Project>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state.projects.clone())
}

#[tauri::command]
pub fn create_project(
    state: State<'_, AppStateMutex>,
    name: String,
    color: Option<String>,
) -> Result<Project, String> {
    let desktop = desktop_manager::create_desktop()?;
    let now = chrono::Utc::now().to_rfc3339();

    // Rename the desktop to match project name
    desktop_manager::rename_desktop(desktop.index, &name)?;

    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.clone(),
        desktop_guid: Some(desktop.guid),
        desktop_name: Some(name),
        color,
        created_at: now.clone(),
        updated_at: now,
    };

    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.projects.push(project.clone());
    save_state(&state)?;

    Ok(project)
}

#[tauri::command]
pub fn delete_project(
    state: State<'_, AppStateMutex>,
    project_id: String,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let project = state
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or("Project not found")?
        .clone();

    // Remove the virtual desktop if it exists
    if project.desktop_guid.is_some() {
        let desktops = desktop_manager::list_desktops()?;
        if let Some(desktop) = desktops.iter().find(|d| Some(&d.guid) == project.desktop_guid.as_ref()) {
            if desktops.len() > 1 {
                let _ = desktop_manager::remove_desktop(desktop.index);
            }
        }
    }

    // Remove assignments for this project
    state.assignments.retain(|a| a.project_id != project_id);
    state.projects.retain(|p| p.id != project_id);
    save_state(&state)?;

    Ok(())
}

#[tauri::command]
pub fn rename_project(
    state: State<'_, AppStateMutex>,
    project_id: String,
    name: String,
) -> Result<Project, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let project = state
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or("Project not found")?;

    project.name = name.clone();
    project.desktop_name = Some(name.clone());
    project.updated_at = chrono::Utc::now().to_rfc3339();

    // Rename the virtual desktop too
    if project.desktop_guid.is_some() {
        let desktops = desktop_manager::list_desktops()?;
        if let Some(desktop) = desktops.iter().find(|d| Some(&d.guid) == project.desktop_guid.as_ref()) {
            let _ = desktop_manager::rename_desktop(desktop.index, &name);
        }
    }

    let project = project.clone();
    save_state(&state)?;

    Ok(project)
}

#[tauri::command]
pub fn switch_project(
    state: State<'_, AppStateMutex>,
    project_id: String,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;

    let project = state
        .projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or("Project not found")?;

    let desktop_guid = project
        .desktop_guid
        .as_ref()
        .ok_or("Project has no desktop")?;

    let desktops = desktop_manager::list_desktops()?;
    let desktop = desktops
        .iter()
        .find(|d| &d.guid == desktop_guid)
        .ok_or("Desktop not found")?;

    desktop_manager::switch_to_desktop(desktop.index)
}

fn save_state(state: &crate::state::AppState) -> Result<(), String> {
    let data = PersistenceData {
        projects: state.projects.clone(),
        assignments: state.assignments.clone(),
    };
    persistence::save_data(&state.data_path, &data)
}
