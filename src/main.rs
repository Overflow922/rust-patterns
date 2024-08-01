use dyn_decorator::{LoggingService, WallClockService};
use service::{RoomService, Service};
use static_decorator::{LoggingServiceSstatic, WallClockServiceStatic};

mod dyn_decorator;
mod service;
mod static_decorator;

fn main() {
    dyn_decorator();
    static_decorator();
}

fn static_decorator() {
    println!("doing static.");
    let chain = WallClockServiceStatic::new(LoggingServiceSstatic::new(RoomService::default()));
    chain.service();
}

fn dyn_decorator() {
    println!("doing dynamic.");
    let chain = WallClockService::new(Box::new(LoggingService::new(Box::new(
        RoomService::default(),
    ))));
    chain.service();
}
