use amethyst::ecs::Entity;
use std::collections::VecDeque;

use crate::constants::InputType;

#[derive(Default)]
pub struct TurnState {
  pub waiting: bool,
  pub turn_queue: VecDeque<Entity>,
  pub queued_move: Option<InputType>,
}
