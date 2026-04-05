use serde::{Deserialize, Serialize};

fn default_board_status() -> String {
    "todo".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub desktop_guid: Option<String>,
    pub desktop_name: Option<String>,
    pub color: Option<String>,
    #[serde(default)]
    pub sort_order: u32,
    #[serde(default = "default_board_status")]
    pub board_status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowAssignment {
    pub project_id: String,
    pub window_handle: isize,
    pub window_title: String,
    pub exe_name: String,
    pub assigned_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub handle: isize,
    pub title: String,
    pub exe_name: String,
    pub desktop_id: Option<String>,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopInfo {
    pub guid: String,
    pub name: String,
    pub index: u32,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceData {
    pub projects: Vec<Project>,
    pub assignments: Vec<WindowAssignment>,
    #[serde(default)]
    pub tasks: Vec<Task>,
}
