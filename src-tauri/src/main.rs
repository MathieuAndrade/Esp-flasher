// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serialport;
use std::{thread, time};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{LogicalSize, Size, Window};

pub mod progress;

mod flasher;
use flasher::flash;

#[tauri::command]
fn list_ports() -> Vec<String> {
    let mut ports = Vec::new();
    serialport::available_ports()
        .expect("No ports found!")
        .iter()
        .for_each(|port| {
            ports.push(port.port_name.clone());
        });

    ports
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
    flash(window, port, 0x0, 921600, file);
}

#[tauri::command]
fn flash_image(window: Window, port: String, file: String) {
    println!("Flashing image on port: {}", port);
    flash(window, port, 3145728, 921600, file);
}

#[tauri::command]
fn set_window_size(window: Window, target_width: f64, target_height: f64, operation_count: f64) {
    // Get the current window size
    let mut current_width: f64 = window.inner_size().unwrap().width as f64;
    let mut current_height: f64 = window.inner_size().unwrap().height as f64;

    // Calculate the interval for each operation
    // this is useful to prevent the window from resizing too fast/slow
    // and to much cpu usage
    let width_interval: f64;
    let height_interval: f64;

    if current_width < target_width {
        width_interval = (target_width - current_width) / operation_count;
    } else {
        width_interval = (current_width - target_width) / operation_count;
    }

    if current_height < target_height {
        height_interval = (target_height - current_height) / operation_count;
    } else {
        height_interval = (current_height - target_height) / operation_count;
    }

    // Spawn a thread to resize the window
    thread::spawn(move || {
        while current_width != target_width || current_height != target_height {
            // Check what operation is needed to be done on the window width
            // and set the new width of the window
            if current_width < target_width {
                current_width = current_width + width_interval;
            } else if current_width > target_width {
                current_width = current_width - width_interval;
            }

            // Check what operation is needed to be done on the window height
            // and set the new height of the window
            if current_height < target_height {
                current_height = current_height + height_interval;
            } else if current_height > target_height {
                current_height = current_height - height_interval;
            }

            // Set the new size of the window
            window
                .set_size(Size::Logical(LogicalSize {
                    width: current_width,
                    height: current_height,
                }))
                .unwrap();

            // Sleep for 20ms to prevent the thread from using too much CPU
            thread::sleep(time::Duration::from_millis(20));
        }
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_ports,
            flash_firmware,
            flash_image,
            open_file_dialog,
            set_window_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
