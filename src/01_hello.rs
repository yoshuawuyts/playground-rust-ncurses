extern crate ncurses;

fn main() {
  ncurses::initscr();
  ncurses::printw("oi mate!");
  ncurses::refresh();
  ncurses::getch();
  ncurses::endwin();
}
