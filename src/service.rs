pub trait Service {
    fn service(&self);
}

#[derive(Default)]
pub struct RoomService {}

impl Service for RoomService {
    fn service(&self) {
        println!("doing room service");
    }
}
