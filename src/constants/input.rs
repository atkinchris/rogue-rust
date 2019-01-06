pub enum InputType {
  Empty,
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,
}

impl Default for InputType {
  fn default() -> InputType {
    InputType::Empty
  }
}
