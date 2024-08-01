use command::{MoveCommand, RotateCommand};
use game_world::{GameWorld, World};
use strategy::BaseTranslator;
use strategy::InverseLang;
use strategy::Translator;

mod command;
mod game_world;
mod strategy;

fn main() {
    println!("command pattern:");
    let world = GameWorld::default();
    world.apply_command("name", MoveCommand::default());
    world.apply_command("name", RotateCommand::default());

    println!();
    println!("strategy pattern");
    let translator = BaseTranslator::default();
    let result = translator.translate("123", &InverseLang::default());
    println!("result is {result}");
}
