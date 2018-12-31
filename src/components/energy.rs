use amethyst::ecs::{Component, VecStorage};

pub struct Energy {
  pub energy: i32,
  pub speed: i32,
}

impl Energy {
  pub fn new() -> Energy {
    Energy {
      energy: 0,
      speed: 5,
    }
  }

  pub fn is_ready(&self) -> bool {
    self.energy >= self.speed
  }
}

impl Component for Energy {
  type Storage = VecStorage<Self>;
}
