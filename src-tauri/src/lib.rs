use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn pick_tmod_dir(app: tauri::AppHandle) -> Option<String> {
	let path = app.dialog().file().blocking_pick_folder();
	println!("Picked tModLoader directory: {:?}", path);

	if let Some(dir) = path {
		Some(dir.to_string())
	} else {
		None
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let mut builder = tauri::Builder::default()
		.plugin(tauri_plugin_fs::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_store::Builder::new().build())
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![greet, pick_tmod_dir]);

	#[cfg(debug_assertions)]
	{
		builder = builder.plugin(tauri_plugin_devtools::init());
	}

	builder
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
