#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_sql::TauriSql;

#[tauri::command]
async fn get_avg_ambient(){
}

fn main() {
  tauri::Builder::default()
    .plugin(TauriSql::default())
    .invoke_handler(tauri::generate_handler![get_avg_ambient])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
