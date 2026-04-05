use std::sync::Mutex;

use crate::models::{Project, Task, WindowAssignment};

#[derive(Default)]
pub struct AppState {
    pub projects: Vec<Project>,
    pub assignments: Vec<WindowAssignment>,
    pub tasks: Vec<Task>,
    pub data_path: std::path::PathBuf,
}

pub type AppStateMutex = Mutex<AppState>;
