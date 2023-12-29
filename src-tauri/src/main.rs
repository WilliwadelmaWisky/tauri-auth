// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;

#[tauri::command]
fn signin(username: &str, password: &str) -> bool {
    return auth::is_valid_user(username, password);
}

fn main() {
    auth::initialize();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![signin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
