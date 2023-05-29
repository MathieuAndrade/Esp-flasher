// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::Window;
extern crate serial_enumerate;

#[derive(Clone, serde::Serialize)]
struct Payload {
    std: String,
}

macro_rules! esp_cmd {
    () => {
        ".\\esptool.exe --before default_reset --after hard_reset --port {} --baud {} write_flash {} {}"
    }
}

fn run_command(window: Window, cmd: &str) {
    let mut cmd = Command::new("cmd")
        .current_dir("../")
        .args(["/C", cmd])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            match line {
                Ok(stdout) => {
                    window
                        .emit(
                            "flashing_progress",
                            Payload {
                                std: stdout.clone(),
                            },
                        )
                        .unwrap();

                    println!("stdout: {}", stdout)
                }
                Err(e) => println!("error parsing header: {e:?}"),
            }
        }
    }

    cmd.wait().unwrap();
}

#[tauri::command]
fn list_ports() -> Vec<String> {
    serial_enumerate::enumerate_serial_ports().unwrap()
}

#[tauri::command]
fn flash_firmware(window: Window, port: &str) {
    println!("Flashing firmware on port: {}", port);

    let cmd = format!(esp_cmd!(), port, 512000, 0x0, ".\\firmware.bin");
    run_command(window, cmd.as_str());
}

#[tauri::command]
fn flash_image(window: Window, port: &str) {
    println!("Flashing image on port: {}", port);

    let cmd = format!(esp_cmd!(), port, 512000, 3145728, ".\\image.bin");
    run_command(window, cmd.as_str());
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
