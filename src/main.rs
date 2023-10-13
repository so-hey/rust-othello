mod othello;
mod textures;

use othello::{Cell, Othello};
use textures::load_textures;
use piston_window::*;

const WINDOW_TITLE: &str = "Othello!";
const WINDOW_SIZE: Size = Size {
    width: 640.0,
    height: 480.0
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(60);
    window.events.set_ups(60);

    let mut bd = Othello::new(Cell::BLACK, load_textures(&mut window));
    let mut pos = [0.0, 0.0];
    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    bd.draw(c, g);
                });
            },
            Event::Loop(Loop::Update(_)) => {
                bd.check();
                // bd.flip(); // player vs COM の場合はこのブロックをコメントアウトする
                if bd.turn == bd.player { // player vs player の場合はこのブロックをコメントアウトする
                    bd.flip();
                } else {
                    bd.put_greedy();
                }
            },
            Event::Input(Input::Button(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Mouse(piston_window::MouseButton::Left),
                scancode: _,
            }), _) => {
                let fx = pos[0]; let fy = pos[1];
                if fx >= 120.0 && fy >= 40.0 {
                    let x = (fx - 120.0) as usize / 50;
                    let y = (fy - 40.0) as usize / 50;
                    bd.select = (y, x);
                }
            },
            Event::Input(Input::Move(Motion::MouseCursor(p)), _) => {
                pos = p;
            }
            _ => {}
        };
    }
}
