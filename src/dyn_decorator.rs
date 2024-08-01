use crate::service::Service;
use std::time::SystemTime;

pub struct LoggingService {
    decorated: Box<dyn Service>,
}

impl LoggingService {
    pub fn new(decorated: Box<dyn Service>) -> Self {
        Self { decorated }
    }
}

impl Service for LoggingService {
    fn service(&self) {
        println!("Hostess entered the room");
        self.decorated.service();
        println!("Hostess left the room");
    }
}

pub struct WallClockService {
    decorated: Box<dyn Service>,
}

impl WallClockService {
    pub fn new(decorated: Box<dyn Service>) -> Self {
        Self { decorated }
    }
}

impl Service for WallClockService {
    fn service(&self) {
        let before = SystemTime::now();
        self.decorated.service();
        match before.elapsed() {
            Ok(elapsed) => println!("Housekeeping took {} ns", elapsed.as_nanos()),
            Err(e) => eprintln!("Failed to measure time for house keeping {e:?}"),
        }
    }
}
