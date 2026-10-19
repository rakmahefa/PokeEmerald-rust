//! Shared contracts used by the Bevy runtime and Dioxus settings application.
//!
//! ROM parsing and game-data extraction intentionally do not live here yet.

use serde::{Deserialize, Serialize};

/// Settings shared across the runtime and desktop settings application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameSettings {
    /// Enable the planned voxel overworld renderer.
    pub voxel_overworld_enabled: bool,
    /// Display the companion information panel in the game window.
    pub kanto_gear_visible: bool,
    /// Main audio level in the inclusive range 0.0..=1.0.
    pub master_volume: f32,
    /// UI scale multiplier, clamped by consumers to a supported range.
    pub ui_scale: f32,
    /// Use fullscreen mode when supported.
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            voxel_overworld_enabled: true,
            kanto_gear_visible: true,
            master_volume: 0.8,
            ui_scale: 1.0,
            fullscreen: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameSettings;

    #[test]
    fn defaults_are_sensible_for_desktop() {
        let settings = GameSettings::default();
        assert!(settings.voxel_overworld_enabled);
        assert!(settings.kanto_gear_visible);
        assert!((0.0..=1.0).contains(&settings.master_volume));
        assert!(settings.ui_scale > 0.0);
        assert!(!settings.fullscreen);
    }

    #[test]
    fn missing_json_fields_use_defaults() {
        let settings: GameSettings = serde_json::from_str("{}").expect("empty settings object");
        assert_eq!(settings, GameSettings::default());
    }
}
