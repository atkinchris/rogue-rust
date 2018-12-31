use amethyst::ecs::{Entities, Join, System, Write, WriteStorage};

use crate::components::{Energy, HasTurn};
use crate::resources::TurnState;

pub struct EnergySystem;

impl<'s> System<'s> for EnergySystem {
  type SystemData = (
    Write<'s, TurnState>,
    Entities<'s>,
    WriteStorage<'s, Energy>,
    WriteStorage<'s, HasTurn>,
  );

  fn run(&mut self, (mut turn_state, entities, mut energies, mut has_turns): Self::SystemData) {
    if !turn_state.waiting {
      for (entity, energy) in (&entities, &mut energies).join() {
        energy.energy += 1;

        if energy.is_ready() {
          has_turns.insert(entity, HasTurn).unwrap();
          turn_state.waiting = true;
          println!("Is Ready!")
        }
      }
    }
  }
}
