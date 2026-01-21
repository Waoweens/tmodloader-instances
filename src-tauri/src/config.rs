use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
use tauri_plugin_store;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Config {
	pub appearance: Appearance,
	pub directories: Directories,
	pub launcher: Launcher,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Appearance {
	pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Directories {
	pub tmodloader_installation: PathBuf,
	pub tmodloader_data: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Launcher {
	pub launch_mode: LaunchMode,
	pub game_arguments: Vec<String>,
	pub wrapper_commands: Vec<String>,
	pub use_dgpu: bool,
	pub use_gamemode: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum Theme {
	System,
	Light,
	Dark,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum LaunchMode {
	Execute,
	SteamRun,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			appearance: Appearance {
				theme: Theme::System,
			},
			directories: Directories {
				tmodloader_installation: PathBuf::new(),
				tmodloader_data: PathBuf::new(),
			},
			launcher: Launcher {
				launch_mode: LaunchMode::Execute,
				game_arguments: Vec::new(),
				wrapper_commands: Vec::new(),
				use_dgpu: false,
				use_gamemode: false,
			},
		}
	}
}
