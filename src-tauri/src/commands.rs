// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::config;

#[tauri::command]
#[specta::specta]
pub fn load_config() -> config::Config {
	config::load_config()
}