use espflash::flasher::ProgressCallbacks;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    msg: String,
}

pub struct Progress {
    pub window: Window,
    pub len: usize,
}

impl ProgressCallbacks for Progress {
    fn init(&mut self, _addr: u32, len: usize) {
        self.window.emit("flash_started", Some(())).unwrap();
        self.len = len;
    }

    fn update(&mut self, current: usize) {
        let msg = (current * 100 / self.len).to_string();
        self.window
            .emit("flash_progress_update", Payload { msg })
            .unwrap();
    }

    fn finish(&mut self) {
        self.window.emit("flash_finished", Some(())).unwrap();
    }
}
