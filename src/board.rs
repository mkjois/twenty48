use std::string;
use util;
use board::Action::*;
use board::Status::*;

pub struct Board {
  l: usize,
  w: usize,
  goal: u32,
  cells: Vec<u32>,
}

#[derive(Copy)]
pub enum Action {
  Up,
  Down,
  Left,
  Right,
  Noop,
}

#[derive(Copy)]
pub enum Status {
  Valid,
  Invalid,
  Win,
  Lose,
}

impl Board {
  pub fn new(l: usize, w: usize, goal: u32) -> Board {
    let mut cells = Vec::with_capacity(l*w);
    for _ in 0..l*w {
      cells.push(0);
    }
    let mut b = Board{l: l, w: w, goal: goal, cells: cells};
    b.spawn(0.5);
    return b;
  }

  // spawns a tile where 'prob' is the probability of a 4 instead of a 2
  // returns true iff no more valid moves
  fn spawn(&mut self, prob: f32) -> bool {
    let mut index = self.l*self.w;
    let mut denom = 1.0;
    for i in 0..self.l*self.w {
      if self.cells[i] != 0 { continue; }
      if util::flip(1.0 / denom) {
        index = i;
      }
      denom += 1.0;
    }
    if index == self.l*self.w { return true; }
    self.cells[index] = if util::flip(prob) { 4 } else { 2 };
    return false;
  }

  pub fn act(&mut self, action: Action) -> Status {
    let mut changed = false;
    let size = self.l * self.w;
    match action {
      Up => {
        for in_order_j in 0..self.w {
          for in_order_i in 0..self.l {
            let (i, j) = (in_order_i, in_order_j);
            let k = i*self.w + j;
            if self.cells[k] == 0 { continue; }
            let mut tk = k + self.w;
            while tk < size && self.cells[tk] == 0 {
              tk += self.w;
            }
            if tk < size && self.cells[tk] == self.cells[k] {
              // piece below needs to be merged
              // double this cell and delete other cell
              self.cells[k] *= 2;
              self.cells[tk] = 0;
              changed = true;
              if self.cells[k] == self.goal {
                // winner winner chicken dinner
                return Status::Win;
              }
            }
            // move piece upward
            tk = k - self.w;
            while util::check_bounded(tk as i32, 0, (self.l*self.w-1) as i32) && self.cells[tk] == 0 {
              tk -= self.w;
            }
            if tk + self.w != k {
              self.cells[tk + self.w] = self.cells[k];
              self.cells[k] = 0;
              changed = true;
            }
          }
        }
      },
      Down => {
        for in_order_j in 0..self.w {
          for in_order_i in 0..self.l {
            let (i, j) = (self.l - 1 - in_order_i, in_order_j);
            let k = i*self.w + j;
            if self.cells[k] == 0 { continue; }
            let mut tk = k - self.w;
            while util::check_bounded(tk as i32, 0, (self.l*self.w-1) as i32) && self.cells[tk] == 0 {
              tk -= self.w;
            }
            if util::check_bounded(tk as i32, 0, (self.l*self.w-1) as i32) && self.cells[tk] == self.cells[k] {
              // piece above needs to be merged
              // double this cell and delete other cell
              self.cells[k] *= 2;
              self.cells[tk] = 0;
              changed = true;
              if self.cells[k] == self.goal {
                // winner winner chicken dinner
                return Status::Win;
              }
            }
            // move piece downward
            tk = k + self.w;
            while tk < size && self.cells[tk] == 0 {
              tk += self.w;
            }
            if tk - self.w != k {
              self.cells[tk - self.w] = self.cells[k];
              self.cells[k] = 0;
              changed = true;
            }
          }
        }
      },
      Left => {
        for in_order_i in 0..self.l {
          for in_order_j in 0..self.w {
            let (i, j) = (in_order_i, in_order_j);
            let k = i*self.w + j;
            if self.cells[k] == 0 { continue; }
            let mut tk = k + 1;
            while tk / self.w == i && self.cells[tk] == 0 {
              tk += 1;
            }
            if tk / self.w == i && self.cells[tk] == self.cells[k] {
              // piece to the right needs to be merged
              // double this cell and delete other cell
              self.cells[k] *= 2;
              self.cells[tk] = 0;
              changed = true;
              if self.cells[k] == self.goal {
                // winner winner chicken dinner
                return Status::Win;
              }
            }
            // move piece leftward
            tk = k - 1;
            while tk / self.w == i && self.cells[tk] == 0 {
              tk -= 1;
            }
            if tk + 1 != k {
              self.cells[tk + 1] = self.cells[k];
              self.cells[k] = 0;
              changed = true;
            }
          }
        }
      },
      Right => {
        for in_order_i in 0..self.l {
          for in_order_j in 0..self.w {
            let (i, j) = (in_order_i, self.w - 1 - in_order_j);
            let k = i*self.w + j;
            if self.cells[k] == 0 { continue; }
            let mut tk = k - 1;
            while tk / self.w == i && self.cells[tk] == 0 {
              tk -= 1;
            }
            if tk / self.w == i && self.cells[tk] == self.cells[k] {
              // piece to the left needs to be merged
              // double this cell and delete other cell
              self.cells[k] *= 2;
              self.cells[tk] = 0;
              changed = true;
              if self.cells[k] == self.goal {
                // winner winner chicken dinner
                return Status::Win;
              }
            }
            // move piece rightward
            tk = k + 1;
            while tk / self.w == i && self.cells[tk] == 0 {
              tk += 1;
            }
            if tk - 1 != k {
              self.cells[tk - 1] = self.cells[k];
              self.cells[k] = 0;
              changed = true;
            }
          }
        }
      },
      Noop => {},
    }
    if changed {
      let no_more_space = self.spawn(0.5);
      if no_more_space { Status::Lose } else { Status::Valid }
    } else {
      Status::Invalid
    }
  }
}

impl string::ToString for Board {
  fn to_string(&self) -> String {
    let space: usize = 5;
    let mut s = " ".to_string();
    for _ in 0..(space+1)*self.w-1 {
      s.push_str("-");
    }
    s.push_str(" ");
    s.push_str("\n");
    for i in 0..self.l {
      for j in 0..self.w {
        let num = self.cells[(self.w * i + j) as usize];
        let mut num_as_str: &str = &num.to_string();
        if num == 0 {
          num_as_str = "";
        }
        s.push_str("|");
        for _ in 0..space - num_as_str.len() {
          s.push_str(" ");
        }
        s.push_str(num_as_str);
      }
      s.push_str("|\n");
      if i < self.l - 1 {
        for _ in 0..self.w {
          s.push_str("|");
          for _ in 0..space {
            s.push_str("-");
          }
        }
        s.push_str("|\n");
      }
    }
    s.push_str(" ");
    for _ in 0..(space+1)*self.w-1 {
      s.push_str("-");
    }
    s.push_str(" ");
    return s;
  }
}
