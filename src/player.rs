use std::process::Command;

pub struct Player {
    station_url: String,
    child: Option<std::process::Child>,
}

impl Player {
    pub fn new(station_url: String) -> Player {
        Player {
            station_url,
            child: None,
        }
    }
    pub fn play(mut self) -> Player {
        let child = Command::new("mpv")
            .arg(&self.station_url)
            .spawn()
            .expect("failed to execute process");
        self.child = Some(child);
        self
    }
    pub fn stop(mut self) -> Player {
        if let Some(mut value) = self.child {
            value.kill().expect("failed to kill mpv child process");
        };
        self.child = None;
        self
    }
}
