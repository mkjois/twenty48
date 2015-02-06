use ncurses::*;
use board::Board;
use board::Action;

#[derive(Copy)]
pub struct HumanAgent;
#[derive(Copy)]
pub struct RandomAgent;

pub trait Agent {
  fn choose_move(&self, board: &Board, c: i32) -> Action;
}

impl Agent for HumanAgent {
  fn choose_move(&self, _: &Board, c: i32) -> Action {
    match c {
      KEY_DOWN => Action::Down,
      KEY_UP => Action::Up,
      KEY_LEFT => Action::Left,
      KEY_RIGHT => Action::Right,
      _ => Action::Noop,
    }
  }
}

impl Agent for RandomAgent {
  fn choose_move(&self, _: &Board, c: i32) -> Action {
    match c {
      KEY_DOWN => Action::Down,
      KEY_UP => Action::Up,
      KEY_LEFT => Action::Left,
      KEY_RIGHT => Action::Right,
      _ => Action::Noop,
    }
  }
}

pub fn is_arrow_key(c: i32) -> bool {
  c == KEY_DOWN || c == KEY_UP || c == KEY_LEFT || c == KEY_RIGHT
}
