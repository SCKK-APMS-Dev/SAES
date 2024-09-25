use std::{thread, time::Duration};

use active_win_pos_rs::get_active_window;
use tauri::{App, Manager, Window};

#[cfg(desktop)]
mod tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn check_win(app: &mut App) {
    loop {
        let actual = get_active_window().expect("Sikertelen.");
        println!("Actual Window: {:?}", actual.process_path);
        thread::sleep(Duration::from_secs(4));
        let win = app.get_webview_window("main").expect("Nincs");
        win.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
