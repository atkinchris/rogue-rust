use amethyst::ecs::{Read, ReadStorage, System, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{IsPlayer, MovementIntent};
use crate::resources::TurnState;

pub struct IntentSystem;

impl<'s> System<'s> for IntentSystem {
  type SystemData = (
    Write<'s, TurnState>,
    ReadStorage<'s, IsPlayer>,
    Read<'s, InputHandler<StringBindings>>,
    WriteStorage<'s, MovementIntent>,
  );

  fn run(&mut self, (mut turn_state, players, input, mut intents): Self::SystemData) {
    for entity in turn_state.turn_queue.drain(..) {
      if players.get(entity).is_some() {
        intents
          .insert(
            entity,
            MovementIntent {
              x: input.axis_value("player_x").unwrap_or(0.0) as i32,
              y: input.axis_value("player_y").unwrap_or(0.0) as i32,
            },
          )
          .unwrap();
      }
    }
  }
}
