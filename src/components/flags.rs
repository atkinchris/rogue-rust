use amethyst::ecs::{Component, NullStorage};

#[derive(Clone, Debug, Default)]
pub struct IsPlayer;

impl Component for IsPlayer {
    type Storage = NullStorage<Self>;
}
