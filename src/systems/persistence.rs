//! Persistence — save/load game state to disk.

use crate::components::SaveData;

const SAVE_PATH: &str = "save.json";

/// Load save data from disk, or return a default (0) on failure.
pub fn load_save() -> SaveData {
    let Ok(s) = std::fs::read_to_string(SAVE_PATH) else {
        return SaveData {
            high_score: 0,
            levels_beaten: 0,
        };
    };
    serde_json::from_str::<SaveData>(&s).unwrap_or(SaveData {
        high_score: 0,
        levels_beaten: 0,
    })
}

/// Write save data to disk.
pub fn write_save(data: &SaveData) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = std::fs::write(SAVE_PATH, json);
    }
}
