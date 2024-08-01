use crate::game_world::GameObject;

pub trait GameCommand {
    fn execute(&self, object: &mut impl GameObject);
}

#[derive(Default)]
pub struct MoveCommand {}

impl GameCommand for MoveCommand {
    fn execute(&self, object: &mut impl GameObject) {
        println!("apply move command for object");
        object.move_object();
    }
}

#[derive(Default)]
pub struct RotateCommand {}

impl GameCommand for RotateCommand {
    fn execute(&self, object: &mut impl GameObject) {
        println!("apply rotate command for object");
        object.rotate();
    }
}

#[cfg(test)]
mod tests {
    use crate::game_world::MockGameObject;

    use super::*;

    #[test]
    fn when_move_command_then_object_move_called() {
        let mut game_object = MockGameObject::new();
        game_object
            .expect_move_object()
            .with()
            .return_const(())
            .times(1);

        let cmd = MoveCommand::default();
        cmd.execute(&mut game_object);
    }

    #[test]
    fn when_rotate_command_then_object_move_called() {
        let mut game_object = MockGameObject::new();
        game_object.expect_rotate().with().return_const(()).times(1);

        let cmd = RotateCommand::default();
        cmd.execute(&mut game_object);
    }
}
