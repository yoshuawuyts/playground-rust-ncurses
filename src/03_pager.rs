extern crate ncurses as nc;

use std::env;
use std::fs;
use std::path::Path;
use std::io::Read;

fn main() {
  let reader = open_file().bytes();

  // init ncurses
  nc::initscr();
  nc::keypad(nc::stdscr(), true);
  nc::noecho();

  // get screen bounds
  let mut max_y = 0;
  let mut max_x = 0;
  nc::getmaxyx(nc::stdscr(), &mut max_y, &mut max_x);

  // read whole file
  for ch in reader {
    if ch.is_err() {
      break;
    }
    let ch = ch.unwrap();

    // get current screen position
    let mut cur_y = 0;
    let mut cur_x = 0;
    nc::getyx(nc::stdscr(), &mut cur_y, &mut cur_x);

    if cur_y == (max_y - 1) {
      prompt();
      nc::clear();
      nc::mv(0, 0);
    } else {
      nc::addch(ch as nc::chtype);
    }
  }

  // end
  nc::mv(max_y - 1, 0);
  prompt();
  nc::endwin();
}

// open a file
fn open_file () -> fs::File {
  let args: Vec<_> = env::args().collect();
  if args.len() != 2 {
    println!("Usage:\n\t{} <rust file>", args[0]);
    println!("Example:\n\t{} 03_pager.rs", args[0]);
    panic!("Exiting");
  }

  let reader = fs::File::open(Path::new(&args[1]));
  reader.ok().expect("Unable to open file")
}

// prompt for more output
fn prompt () {
  nc::printw("<-Press Any Key->");
  nc::getch();
}
