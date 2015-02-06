#![feature(core,box_syntax)]

extern crate twenty48;
extern crate ncurses;

use twenty48::board::Board;
use twenty48::board::Action;
use twenty48::board::Status;
use twenty48::agent::Agent;
use twenty48::agent::HumanAgent;
//use twenty48::agent::RandomAgent;
use twenty48::agent::is_arrow_key;
use twenty48::util;
use ncurses::*;

fn main() {
  // magic constants
  let (default_l, default_w, default_goal): (u32, u32, u32) = (4, 4, 2048);
  let (lw_lower, lw_upper) = (2, 6);
  let (goal_lower, goal_upper) = (4, 65536);

  let l = util::get_int_input(format!("Board length (default={}, min={}, max={}):", default_l, lw_lower, lw_upper).as_slice(),
                              default_l,
                              |&: x: u32| { util::check_bounded(x as i32, lw_lower, lw_upper) }) as usize;
  let w = util::get_int_input(format!("Board width (default={}, min={}, max={}):", default_w, lw_lower, lw_upper).as_slice(),
                              default_w,
                              |&: x: u32| { util::check_bounded(x as i32, lw_lower, lw_upper) }) as usize;
  let goal = util::get_int_input(format!("Goal power of 2 (default={}, min={}, max={}):", default_goal, goal_lower, goal_upper).as_slice(),
                                 default_goal,
                                 |&: x: u32| { util::check_power_of_base(x, 2) &&
                                               util::check_bounded(x as i32, goal_lower, goal_upper) });
  let agent: Box<Agent> = box HumanAgent;

  /* Setup ncurses. */
  initscr();
  //raw();
  printw(format!("You requested board size {}-by-{} with goal {}\n", l, w, goal).as_slice());

  /* Allow for extended keyboard (like F1). */
  keypad(stdscr, true);
  scrollok(stdscr, true);
  noecho();

  let mut board = Board::new(l, w, goal, 0.5);
  printw(format!("{}\n", board.to_string()).as_slice());

  /* Loop for input. */
  let mut full = false;
  loop {
    refresh();
    if full {
      getch();
      endwin();
      return;
    }
    printw("\nPress an arrow key or 'q' to quit: ");
    let ch = getch();
    if ch as u32 == 'q' as u32 {
      endwin();
      println!("Quitting Twenty48...");
      return;
    } else if is_arrow_key(ch) {
      let action: Action = agent.choose_move(&board, ch);
      let status = board.act(action);
      match status {
        Status::Valid => { printw("\nValid move."); },
        Status::Invalid => { printw("\nNot a valid move."); },
        Status::Win => { printw("\nCongratulations! You've won! Feel free to push it to the limit."); },
        Status::Lose => {
          printw("\nSorry, no more space on the board. You've lost.");
          full = true;
        },
      }
      printw(format!("\n{}", board.to_string()).as_slice());
    } else {
      printw("\nInvalid key.");
      continue;
    }
  }
}
