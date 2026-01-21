use crate::config::Config;
use specta_typescript::Typescript;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::{path::BaseDirectory, Manager};
use tauri_specta::collect_commands;
use toml;

pub mod config;
mod instances;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let specta_builder = tauri_specta::Builder::<tauri::Wry>::new().commands(collect_commands![
		config::get_config,
		config::set_config,
		config::save_config
	]);

	let mut tauri_builder = tauri::Builder::default()
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_store::Builder::new().build())
		.plugin(tauri_plugin_opener::init());

	#[cfg(debug_assertions)]
	{
		tauri_builder = tauri_builder.plugin(tauri_plugin_devtools::init());
	}

	#[cfg(debug_assertions)]
	{
		specta_builder
			.export(Typescript::default(), "../src/lib/bindings.ts")
			.expect("failed to export TypeScript bindings");
	}

	tauri_builder
		.invoke_handler(specta_builder.invoke_handler())
		.setup(move |app| {
			specta_builder.mount_events(app);

			let config_path = app
				.path()
				.resolve(Path::new("config.toml"), BaseDirectory::AppData)
				.unwrap();

			let config_str = fs::read_to_string(&config_path);

			let config: Config = match config_str {
				Ok(content) => toml::from_str(&content).unwrap(),
				Err(_) => {
					println!("config.toml not found, using default config");
					let default_config = Config::default();
					let toml_str = toml::to_string_pretty(&default_config).unwrap();
					fs::create_dir_all(config_path.parent().unwrap()).unwrap();
					fs::write(&config_path, toml_str).unwrap();
					default_config
				}
			};

			app.manage(Mutex::new(config));
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
