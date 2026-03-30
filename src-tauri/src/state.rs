use std::sync::Mutex;

use crate::models::{Project, WindowAssignment};

#[derive(Default)]
pub struct AppState {
    pub projects: Vec<Project>,
    pub assignments: Vec<WindowAssignment>,
    pub data_path: std::path::PathBuf,
}

pub type AppStateMutex = Mutex<AppState>;
