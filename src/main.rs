extern crate ketos;
extern crate pancurses;
extern crate argparse;

use pancurses::*;
use argparse::*;

extern crate es;
use es::*;

fn main() {
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Edit a file with es");
        args.refer(&mut filename)
            .add_argument("file", Store, "File to edit");
        // more state later
        args.parse_args_or_exit();
    }

    let interp = ketos::Interpreter::new();
    match interp.run_file(std::path::Path::new("src/test.ket")) {
        Ok(_) => {},
        Err(e) => {
            println!("oh noes error {}", e);
        }
    }

    let mut es = Editor::new();

    //es.vertical_split();
    //es.horizontal_split();
    //println!("{:?}", es);
    editor_loop(es);
}

fn editor_loop(mut es: Editor) -> i32 {
    let window = initscr();
    window.printw("type things\n");
    window.refresh();
    window.keypad(true);
    noecho();
    let mut buffer: String = "".into();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                buffer.insert(window.get_cur_x() as usize, c);
                es.move_to(window.get_cur_x() as u64 + 1, window.get_cur_y() as u64);
            },
            Some(Input::KeyBackspace) => {
                let x = window.get_cur_x();
                if x != 0 {
                    window.mv(window.get_cur_y(), x - 1);
                    window.delch();
                    buffer.remove(x as usize - 1);
                    es.move_to(x as u64 - 1, window.get_cur_y() as u64);
                }
            },
            Some(Input::KeyLeft) => {
                let x = window.get_cur_x();
                if x != 0 {
                    window.mv(window.get_cur_y(), x - 1);
                    window.refresh();
                    es.move_to(x as u64 - 1, window.get_cur_y() as u64);
                }
            },
            Some(Input::KeyRight) => {
                let x = window.get_cur_x();
                if x != es.current_line().len() as i32 {
                    window.mv(window.get_cur_y(), x + 1);
                    window.refresh();
                    es.move_to(x as u64 + 1, window.get_cur_y() as u64);
                }
            }
            Some(Input::KeyDC) => break,
            Some(input) => { buffer.push_str(&format!("{:?}", input)) },
            None => ()
        }

        window.mvprintw(1, 0, buffer.as_str());
        let (x, y) = es.position();
        window.mv(y as i32, x as i32);
    }
    endwin();
    println!("\"{}\"", buffer);
    3
}

