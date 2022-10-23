#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri_plugin_log::LoggerBuilder;
use tauri_plugin_log::LogTarget;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_sqlite::init())
    .plugin(LoggerBuilder::default().targets([
      LogTarget::LogDir,
      LogTarget::Stdout,
      LogTarget::Webview,
    ]).build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
