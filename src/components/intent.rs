use amethyst::ecs::{Component, VecStorage};

#[derive(Clone, Debug, Default)]
pub struct MovementIntent {
    pub x: i32,
    pub y: i32,
}

impl Component for MovementIntent {
    type Storage = VecStorage<Self>;
}
