extern crate ncurses as nc;

use std::char;

fn main() {
  nc::initscr();
  nc::raw();

  // allow for extended keyboard
  nc::keypad(nc::stdscr(), true);
  nc::noecho();

  // prompt
  nc::printw("Enter a char: ");
  let ch = nc::getch();

  if ch == nc::KEY_F1 {
    nc::attron(nc::A_BOLD() | nc::A_BLINK());
    nc::printw("\nF1");
    nc::attroff(nc::A_BOLD() | nc::A_BLINK());
    nc::printw(" pressed");
  } else {
    nc::printw("\nKey pressed: ");
    nc::attron(nc::A_BOLD() | nc::A_BLINK());
    nc::printw(format!("{}\n", char::from_u32(ch as u32)
      .expect("Invalid char"))
      .as_ref());
    nc::attroff(nc::A_BOLD() | nc::A_BLINK());
  }

  // ueh, get the last input or something
  nc::refresh();

  // end
  nc::getch();
  nc::endwin();
}
