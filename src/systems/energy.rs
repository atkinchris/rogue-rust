use amethyst::ecs::{Entities, Join, System, Write, WriteStorage};

use crate::components::Energy;
use crate::resources::TurnState;

pub struct EnergySystem;

impl<'s> System<'s> for EnergySystem {
  type SystemData = (Write<'s, TurnState>, Entities<'s>, WriteStorage<'s, Energy>);

  fn run(&mut self, (mut turn_state, entities, mut energies): Self::SystemData) {
    if turn_state.turn_queue.is_empty() {
      for (entity, energy) in (&entities, &mut energies).join() {
        energy.energy += 1;

        if energy.is_ready() {
          turn_state.turn_queue.push_back(entity);
          energy.energy = 0;
        }
      }
    }
  }
}
