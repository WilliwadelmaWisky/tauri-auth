// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use auth::AuthInfo;

mod auth;

#[tauri::command]
fn login(username: &str, password: &str) -> bool {
    if auth::is_valid_user(username, password) {
        return auth::authenticate(username);
    }

    return false;
}

#[tauri::command]
fn logout() {
    auth::logout();
}

#[tauri::command]
fn is_authenticated() -> AuthInfo {
    return auth::is_authenticated();
}

#[tauri::command]
fn signup(username: &str, password: &str) -> bool {
    return auth::add_user(username, password);
}

fn main() {
    auth::initialize();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, logout, is_authenticated, signup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
