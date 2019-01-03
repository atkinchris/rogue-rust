use amethyst::core::nalgebra::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::components::MovementIntent;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
  type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, MovementIntent>);

  fn run(&mut self, (mut transforms, movement_intents): Self::SystemData) {
    for (transform, movement) in (&mut transforms, &movement_intents).join() {
      transform.move_global(Vector3::new(
        (movement.x * -16) as f32,
        (movement.y * 16) as f32,
        0.0,
      ));
    }
  }
}
