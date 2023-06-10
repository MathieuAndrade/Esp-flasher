// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::FileDialogBuilder;
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
fn open_file_dialog(window: Window) {
    FileDialogBuilder::default()
        .add_filter("Firmware", &["bin"])
        .pick_file(move |path_buf| match path_buf {
            Some(path) => {
                println!("File selected: {:?}", path);
                window
                    .emit("file_selected", path.to_str().unwrap())
                    .unwrap();
            }
            None => println!("No file selected"),
        })
}

#[tauri::command]
fn flash_firmware(window: Window, port: String, file: String) {
    println!("Flashing firmware on port: {}", port);
    flash(window, port, 0x0, file);
}

#[tauri::command]
fn flash_image(window: Window, port: String, file: String) {
    println!("Flashing image on port: {}", port);
    flash(window, port, 3145728, file);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_ports,
            flash_firmware,
            flash_image,
            open_file_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
