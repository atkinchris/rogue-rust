use amethyst::ecs::{Read, System, Write};
use amethyst::input::InputHandler;

use crate::constants::InputType;
use crate::resources::TurnState;

pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
  type SystemData = (Read<'s, InputHandler<String, String>>, Write<'s, TurnState>);

  fn run(&mut self, (input, mut turn_state): Self::SystemData) {
    let movement = match (
      input.axis_value("player_x").unwrap_or(0.0) as i32,
      input.axis_value("player_y").unwrap_or(0.0) as i32,
    ) {
      (-1, 0) => Some(InputType::MoveLeft),
      (1, 0) => Some(InputType::MoveRight),
      (0, 1) => Some(InputType::MoveUp),
      (0, -1) => Some(InputType::MoveDown),
      (_, _) => None,
    };

    turn_state.queued_move = movement;
  }
}
