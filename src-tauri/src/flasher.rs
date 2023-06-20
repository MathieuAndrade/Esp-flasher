use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::thread;
use tauri::Window;

use espflash::cli::{config::Config, connect, ConnectArgs};

use crate::progress::Progress;

pub fn flash(window: Window, port: String, addr: u32, baud: u32, bin_file: String) {
    let args = ConnectArgs {
        port: Some(port),
        baud: Some(baud),
        no_stub: false,
    };

    let config = Config::load().unwrap();
    let mut flasher = connect(&args, &config).expect("cannot connect to board");

    // print_board_info(&mut flasher).unwrap();

    let file_exist = Path::new(&bin_file).exists();

    if file_exist {
        let mut f = File::open(&bin_file).unwrap();

        let metadata = fs::metadata(&bin_file).unwrap();
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).unwrap();

        thread::spawn(move || {
            let mut pg = Progress { window, len: 0 };

            flasher
                .write_bin_to_flash(addr, &buffer, Some(&mut pg))
                .ok()
                .expect("error durring flash");
        });
    } else {
        window.emit("file_not_found", Some(())).unwrap();
    }
}
