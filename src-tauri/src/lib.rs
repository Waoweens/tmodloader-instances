use serde::{Deserialize, Serialize};
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

pub mod config;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let specta_builder = Builder::<tauri::Wry>::new()
		.commands(collect_commands![commands::load_config]);

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
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
		
}
