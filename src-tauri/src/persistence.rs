use std::path::Path;

use crate::models::PersistenceData;

pub fn load_data(path: &Path) -> PersistenceData {
    if path.exists() {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or(PersistenceData {
            projects: Vec::new(),
            assignments: Vec::new(),
            tasks: Vec::new(),
        })
    } else {
        PersistenceData {
            projects: Vec::new(),
            assignments: Vec::new(),
            tasks: Vec::new(),
        }
    }
}

pub fn save_data(path: &Path, data: &PersistenceData) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}
