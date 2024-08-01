use command::{MoveCommand, RotateCommand};
use game_world::{GameWorld, World};

mod command;
mod game_world;

fn main() {
    let world = GameWorld::default();
    world.apply_command("name", MoveCommand::default());
    world.apply_command("name", RotateCommand::default());
}
