use coord::{Coord, Dir};
use game::Game;
use std::io::{stdout, Read, Write, Bytes};
use std::iter;
use std::thread;
use std::time::Duration;
use termion as tm;
use termion::{async_stdin, AsyncReader};
use termion::raw::IntoRawMode;

struct State {
    field_pos: Coord,
}

pub fn play(mut game: Game) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(
        stdout,
        "{}{}{} -TEXTRIS-",
        tm::clear::All,
        tm::cursor::Goto(1, 1),
        tm::cursor::Hide,
    ).unwrap();

    let state = State {
        field_pos: Coord(1, 2),
    };

    let interval = Duration::from_millis(32);
    let mut t = 0;
    loop {
        match stdin.next() {
            Some(Ok(key)) => match key {
                b'q' => break,
                b'h' => game.slide_piece(Dir::Left),
                b'l' => game.slide_piece(Dir::Right),
                b'j' => game.slide_piece(Dir::Down),
                b'd' => game.rotate_piece(false),
                b'f' => game.rotate_piece(true),
                _ => {}
            },
            _ => {}
        }

        if t % 20 == 0 {
            match game.tick() {
                Ok(_) => {}
                Err(_) => {
                    render_game_over(&mut stdout);
                    stdout.flush().unwrap();
                    wait_any_key_input(&mut stdin, interval);
                    break;
                }
            }
        }

        render(&mut stdout, &game, &state);
        stdout.flush().unwrap();

        thread::sleep(interval);
        t += 1;
    }

    write!(stdout, "{}", tm::cursor::Show).unwrap();
}

fn render(w: &mut Write, g: &Game, state: &State) {
    let field = g.field();
    let Coord(x, y) = state.field_pos;
    let x = x as usize;
    let y = y as usize;

    for (i, line) in field.lines_iter().enumerate() {
        write!(w, "{}|", tm::cursor::Goto(x as u16, (i + y) as u16)).unwrap();
        for cell in line.iter() {
            match cell {
                Some(block) => write!(w, "{} ", block.chr),
                None => write!(w, "  "),
            }.unwrap();
        }
        write!(w, "|").unwrap();
    }

    write!(w, "{}", tm::cursor::Goto(x as u16, (field.height() + y) as u16)).unwrap();
    let width = field.width();
    for floor in iter::repeat("--").take(width + 1) {
        write!(w, "{}", floor).unwrap();
    }
}

fn render_game_over(w: &mut Write) {
    write!(w, "{}", tm::cursor::Goto(1, 1)).unwrap();
    write!(w, "====== GAME OVER ======").unwrap();
}

fn wait_any_key_input(stdin: &mut Bytes<AsyncReader>, interval: Duration) {
    loop {
        match stdin.next() {
            Some(_) => break,
            _ => {}
        }
        thread::sleep(interval);
    }
}