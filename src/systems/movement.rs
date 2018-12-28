use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::IsPlayer;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, IsPlayer>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
    for (_, transform) in (&players, &mut transforms).join() {
      let movement = (input.axis_value("player_x"), input.axis_value("player_y"));

      println!("Movement {:?}", movement);
    }
  }
}
