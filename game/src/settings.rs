use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

#[derive(Deserialize)]
pub struct ControlBindings {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
}

#[derive(Deserialize)]
pub struct GameSettings {
    pub window: WindowSettings,
    pub controls: ControlBindings,
}

impl GameSettings {
    pub fn load() -> Self {
        let mut path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        path.push("settings.toml");

        let content = fs::read_to_string(&path).expect("Failed to read settings.toml");
        toml::from_str(content.as_str()).expect("Failed to parse settings.toml")
    }
}
