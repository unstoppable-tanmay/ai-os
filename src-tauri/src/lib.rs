use std::env;

use std::sync::Mutex;

use tauri::Manager;

#[derive(Default)]
struct AppState {
    counter: u32,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn os_name() -> &'static str {
    env::consts::OS
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            let _state = app.state::<Mutex<AppState>>();
            let y = _state.lock().unwrap();
            println!("{}", y.counter);
            println!("{}", os_name());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
