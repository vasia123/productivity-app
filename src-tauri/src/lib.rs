mod commands;
mod desktop_manager;
mod models;
mod persistence;
mod state;
mod window_enum;

use state::AppState;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use winvd::DesktopEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            let data_path = data_dir.join("projects.json");

            let data = persistence::load_data(&data_path);

            let mut app_state = AppState {
                projects: data.projects,
                assignments: data.assignments,
                tasks: data.tasks,
                data_path,
            };

            // Sync with existing Windows virtual desktops
            if let Err(e) = commands::projects::sync_desktops(&mut app_state) {
                eprintln!("Warning: failed to sync desktops: {e}");
            } else {
                let persist_data = models::PersistenceData {
                    projects: app_state.projects.clone(),
                    assignments: app_state.assignments.clone(),
                    tasks: app_state.tasks.clone(),
                };
                let _ = persistence::save_data(&app_state.data_path, &persist_data);
            }

            app.manage(Mutex::new(app_state));

            // Pin app window to all virtual desktops
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(hwnd) = window.hwnd() {
                    let hwnd = windows::Win32::Foundation::HWND(hwnd.0);
                    let _ = winvd::pin_window(hwnd);
                }
            }

            // Listen for virtual desktop switches and emit event to frontend
            let app_handle = app.handle().clone();
            let (tx, rx) = std::sync::mpsc::channel::<DesktopEvent>();
            let _event_thread = winvd::listen_desktop_events(tx);
            // Keep the event thread alive by storing it in managed state
            app.manage(_event_thread);

            std::thread::spawn(move || {
                for event in rx {
                    if let DesktopEvent::DesktopChanged { new, .. } = event {
                        let new_guid = format!("{:?}", new.get_id().unwrap_or_default());

                        // Update board_status in state
                        if let Some(state_mutex) = app_handle.try_state::<Mutex<AppState>>() {
                            if let Ok(mut state) = state_mutex.lock() {
                                let now = chrono::Utc::now().to_rfc3339();
                                let mut changed = false;

                                for p in &mut state.projects {
                                    if p.desktop_guid.as_ref() == Some(&new_guid) {
                                        if p.board_status != "in_progress" {
                                            p.board_status = "in_progress".into();
                                            p.updated_at = now.clone();
                                            changed = true;
                                        }
                                    } else if p.board_status == "in_progress" {
                                        p.board_status = "todo".into();
                                        p.updated_at = now.clone();
                                        changed = true;
                                    }
                                }

                                if changed {
                                    let _ = commands::projects::save_state(&state);
                                }
                            }
                        }

                        let _ = app_handle.emit("desktop-changed", &new_guid);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::projects::list_projects,
            commands::projects::create_project,
            commands::projects::delete_project,
            commands::projects::rename_project,
            commands::projects::switch_project,
            commands::windows::list_open_windows,
            commands::windows::assign_window_to_project,
            commands::windows::unassign_window,
            commands::windows::get_project_windows,
            commands::windows::kill_window_process,
            commands::windows::get_other_project_windows,
            commands::windows::list_all_windows,
            commands::projects::import_desktops,
            commands::projects::set_project_board_status,
            commands::projects::reorder_projects,
            commands::tasks::create_task,
            commands::tasks::delete_task,
            commands::tasks::update_task_status,
            commands::tasks::list_tasks,
            commands::desktops::list_desktops,
            commands::desktops::get_current_desktop,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Save state before exit
                if let Some(state) = window.try_state::<Mutex<AppState>>() {
                    if let Ok(state) = state.lock() {
                        let _ = commands::projects::save_state(&state);
                    }
                }
                window.app_handle().exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
