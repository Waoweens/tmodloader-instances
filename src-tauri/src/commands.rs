use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn pick_tmod_dir(app: tauri::AppHandle) -> Option<String> {
	let path = app.dialog().file().blocking_pick_folder();
	println!("Picked tModLoader directory: {:?}", path);

	if let Some(dir) = path {
		Some(dir.to_string())
	} else {
		None
	}
}