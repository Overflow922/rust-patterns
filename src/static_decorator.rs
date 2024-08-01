use crate::service::Service;
use std::time::SystemTime;

pub struct LoggingServiceSstatic<R: Service> {
    decorated: R,
}

impl<R: Service> LoggingServiceSstatic<R> {
    pub fn new(decorated: R) -> Self {
        Self { decorated }
    }
}

impl<R: Service> Service for LoggingServiceSstatic<R> {
    fn service(&self) {
        println!("Hostess entered the room");
        self.decorated.service();
        println!("Hostess left the room");
    }
}

pub struct WallClockServiceStatic<R: Service> {
    decorated: R,
}

impl<R: Service> WallClockServiceStatic<R> {
    pub fn new(decorated: R) -> Self {
        Self { decorated }
    }
}

impl<R: Service> Service for WallClockServiceStatic<R> {
    fn service(&self) {
        let before = SystemTime::now();
        self.decorated.service();
        match before.elapsed() {
            Ok(elapsed) => println!("Housekeeping took {} ns", elapsed.as_nanos()),
            Err(e) => eprintln!("Failed to measure time for house keeping {e:?}"),
        }
    }
}
