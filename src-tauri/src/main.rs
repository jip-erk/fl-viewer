// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn run_shell_command(fl: &str, beats: &str, destination: &str) -> Result<String, String> {
    let output = Command::new(fl)
        .arg("/R")
        .arg("/Emp3")
        .arg(format!("/F{}", beats))
        .arg(format!("/O{}", destination))
        .output()
        .map_err(|err| format!("Failed to execute command: {}", err))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_shell_command])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
