// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod kicad_ipc;

use kicad_ipc::KiCadInstance;

#[tauri::command]
fn echo_message(message: &str) -> String {
    format!("Echo: {}", message)
}

#[tauri::command]
fn detect_kicad_instances() -> Vec<KiCadInstance> {
    kicad_ipc::detect_kicad_instances()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![echo_message, detect_kicad_instances])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
