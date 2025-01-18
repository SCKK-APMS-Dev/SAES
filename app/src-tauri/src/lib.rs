use std::{thread, time::Duration};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let overlay = app.get_webview_window("overlay").unwrap();
            let loader = app.get_webview_window("loader").unwrap();
            let main = app.get_webview_window("main").unwrap();
            loader.show().unwrap();
            loader.set_focus().unwrap();
            thread::sleep(Duration::from_secs(5));
            loader.hide().unwrap();
            overlay.show().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
