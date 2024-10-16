use std::thread::spawn;

use active_win_pos_rs::get_active_window;
mod rpc;
#[cfg(desktop)]
mod tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_win() -> String {
    let actual = get_active_window().expect("Sikertelen.");
    format!("{:?}", actual.process_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    spawn(rpc::main);
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, check_win])
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
