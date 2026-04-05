use tauri::State;

use crate::models::Task;
use crate::state::AppStateMutex;

use super::projects::save_state;

#[tauri::command]
pub fn create_task(
    state: State<'_, AppStateMutex>,
    project_id: String,
    title: String,
) -> Result<Task, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        project_id,
        title,
        status: "todo".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
        completed_at: None,
    };

    state.tasks.push(task.clone());
    save_state(&state)?;

    Ok(task)
}

#[tauri::command]
pub fn delete_task(
    state: State<'_, AppStateMutex>,
    task_id: String,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.tasks.retain(|t| t.id != task_id);
    save_state(&state)?;
    Ok(())
}

#[tauri::command]
pub fn update_task_status(
    state: State<'_, AppStateMutex>,
    task_id: String,
    status: String,
) -> Result<Task, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let task = state
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or("Task not found")?;

    task.status = status.clone();
    task.completed_at = if status == "done" {
        Some(chrono::Utc::now().to_rfc3339())
    } else {
        None
    };

    let task = task.clone();
    save_state(&state)?;

    Ok(task)
}

#[tauri::command]
pub fn list_tasks(
    state: State<'_, AppStateMutex>,
    project_id: String,
) -> Result<Vec<Task>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state
        .tasks
        .iter()
        .filter(|t| t.project_id == project_id)
        .cloned()
        .collect())
}
