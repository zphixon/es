extern crate ketos;
extern crate pancurses;
extern crate ropey;

use pancurses::*;

fn main() {
    let interp = ketos::Interpreter::new();
    interp.run_file(std::path::Path::new("src/test.ket")).unwrap();

    let window = initscr();
    cbreak();
    noecho();
    window.keypad(true);
    window.printw("Type things! Press delete to quit\n");
    window.refresh();
    loop {
        match window.getch() {
            Some(Input::KeyDC) => break,
            Some(Input::KeyBackspace) => {
                window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                window.delch();
            },
            Some(Input::KeyLeft) => {
                window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                window.refresh();
            },
            Some(Input::KeyRight) => {
                window.mv(window.get_cur_y(), window.get_cur_x() + 1);
                window.refresh();
            },
            Some(Input::KeyUp) => {
                window.mv(window.get_cur_y() - 1, window.get_cur_x());
                window.refresh();
            },
            Some(Input::KeyDown) => {
                window.mv(window.get_cur_y() + 1, window.get_cur_x());
                window.refresh();
            },
            Some(Input::Character(c)) => {
                window.addch(c);
            },
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            },
            None => ()
        }
    }
    endwin();

    let mut test_rope = ropey::Rope::from_str("Hello, world! This is a test of a rope data structure. Pretty neat!");
    test_rope.remove_text_between_char_indices(14, 55);
    test_rope.append(ropey::Rope::from_str(" Hey, not bad!"));
    println!("{}", test_rope.to_string());
}

