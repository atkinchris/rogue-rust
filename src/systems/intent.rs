use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{HasTurn, IsPlayer, MovementIntent};

pub struct IntentSystem;

impl<'s> System<'s> for IntentSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, HasTurn>,
    ReadStorage<'s, IsPlayer>,
    Read<'s, InputHandler<String, String>>,
    WriteStorage<'s, MovementIntent>,
  );

  fn run(&mut self, (entities, mut has_turns, players, input, mut intents): Self::SystemData) {
    for (entity) in (&entities).join() {
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

    has_turns.clear();
  }
}
