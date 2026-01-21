// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::config::Config;
use std::{path::Path, sync::Mutex, fs};
use tauri::{Manager, path::BaseDirectory};

#[tauri::command]
#[specta::specta]
pub fn get_config(app: tauri::AppHandle) -> Config {
	println!("get_config called");
	app.state::<Mutex<Config>>().lock().unwrap().clone()
}

#[tauri::command]
#[specta::specta]
pub fn set_config(app: tauri::AppHandle, new_config: Config) {
	println!("set_config called");
	let config_state = app.state::<Mutex<Config>>();
	let mut config = config_state.lock().unwrap();
	*config = new_config;
}

#[tauri::command]
#[specta::specta]
pub fn save_config(app: tauri::AppHandle) {
	println!("save_config called");
	let config = app.state::<Mutex<Config>>().lock().unwrap().clone();
	let config_path = app
		.path()
		.resolve(Path::new("config.toml"), BaseDirectory::AppData)
		.unwrap();

	let toml_str = toml::to_string_pretty(&config).unwrap();
	fs::write(&config_path, toml_str).unwrap();
}