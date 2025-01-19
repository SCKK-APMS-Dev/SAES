use tauri::Manager;

#[tauri::command]
async fn update_done(app_handle: tauri::AppHandle) {
    let overlay = app_handle.get_webview_window("overlay").unwrap();
    let loader = app_handle.get_webview_window("loader").unwrap();
    loader.hide().unwrap();
    overlay.show().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![update_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
