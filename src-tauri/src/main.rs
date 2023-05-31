// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

extern crate serial_enumerate;

pub mod progress;

mod flasher;
use flasher::flash;

#[tauri::command]
fn list_ports() -> Vec<String> {
    serial_enumerate::enumerate_serial_ports().unwrap()
}

#[tauri::command]
fn flash_firmware(window: Window, port: String) {
    println!("Flashing firmware on port: {}", port);
    flash(window, port, 0x0, "..\\firmware.bin".to_string());
}

#[tauri::command]
fn flash_image(window: Window, port: String) {
    println!("Flashing image on port: {}", port);
    flash(window, port, 3145728, "..\\image.bin".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_ports,
            flash_firmware,
            flash_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
