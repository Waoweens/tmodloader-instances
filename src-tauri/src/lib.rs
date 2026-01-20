mod commands;
mod config;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let mut builder = tauri::Builder::default()
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_store::Builder::new().build())
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			commands::greet,
			commands::pick_tmod_dir,
		]);

	#[cfg(debug_assertions)]
	{
		builder = builder.plugin(tauri_plugin_devtools::init());
	}

	builder
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
