use amethyst::ecs::{Component, NullStorage};

#[derive(Clone, Debug, Default)]
pub struct IsPlayer;

impl Component for IsPlayer {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct HasTurn;

impl Component for HasTurn {
    type Storage = NullStorage<Self>;
}
