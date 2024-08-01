use mockall::automock;

use crate::command::GameCommand;

#[automock]
pub trait GameObject {
    fn move_object(&mut self);
    fn rotate(&mut self);
}

pub struct Object {}

impl GameObject for Object {
    fn move_object(&mut self) {
        println!("moving object");
    }

    fn rotate(&mut self) {
        println!("rotating object");
    }
}

pub trait World {
    fn get_object(&self, name: &str) -> impl GameObject;
    fn apply_command(&self, name: &str, cmd: impl GameCommand);
}

#[derive(Default)]
pub struct GameWorld {}

impl World for GameWorld {
    fn get_object(&self, _name: &str) -> impl GameObject {
        Object {}
    }

    fn apply_command(&self, name: &str, cmd: impl GameCommand) {
        println!("Getting obj by name {name}");
        let mut obj = self.get_object(name);
        cmd.execute(&mut obj);
    }
}
