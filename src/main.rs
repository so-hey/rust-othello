mod othello;
mod textures;
// use std::{fs::OpenOptions, path::Path};

use othello::{Cell, Othello};
use textures::load_textures;
use piston_window::*;

// struct App {
//     othello: Othello,
//     elapsed_time: f64,
//     glyphs: piston_window::Glyphs,
//     pos: [f64; 2]
// }

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
                bd.flip();
                // if bd.turn == bd.player {
                //     bd.flip();
                // } else {
                //     bd.put_greedy();
                // }
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

// fn start_app() {
//     let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
//         .exit_on_esc(true)
//         .vsync(true)
//         .resizable(true)
//         .samples(4)
//         .build()
//         .unwrap_or_else(|e| panic!("falied to build PistonWindow: {}", e));
//     window.events.max_fps(60);
//     window.events.set_ups(60);

//     let font_path = match OpenOptions::new().read(true).open("ipag.tff") {
//         Ok(_) => Path::new("ipag.tff"),
//         Err(_) => {
//             match OpenOptions::new().read(true).open("src/ipag.tff") {
//                 Ok(_) => Path::new("src/ipag.tff"),
//                 Err(_) => panic!("Font file is missing, or does not exist in the current path"),
//             }
//         }
//     };

//     let mut app = App {
//         othello: Othello::new(Cell::BLACK, load_textures(&mut window)),
//         elapsed_time: 0.0,
//         glyphs: window.load_font(font_path).unwrap(),
//         pos: [0.0, 0.0]
//     };

//     window.set_lazy(false);
//     while let Some(e) = window.next() {
//         match e {
//             Event::Loop(Loop::Render(_)) => {
//                 app.render(&mut window, &e);
//             },
//             Event::Loop(Loop::Update(_)) => {
//                 bd.check();
//                 if bd.turn == bd.player {
//                     bd.flip();
//                 } else {
//                     bd.put_greedy();
//                 }
//             },
//             Event::Input(Input::Button(ButtonArgs {
//                 state: ButtonState::Press,
//                 button: Button::Mouse(piston_window::MouseButton::Left),
//                 scancode: _,
//             }), _) => {
//                 let fx = pos[0]; let fy = pos[1];
//                 if fx >= 120.0 && fy >= 40.0 {
//                     let x = (fx - 120.0) as usize / 50;
//                     let y = (fy - 40.0) as usize / 50;
//                     bd.select = (y, x);
//                 }
//             },
//             Event::Input(Input::Move(Motion::MouseCursor(p)), _) => {
//                 pos = p;
//             }
//             _ => {}
//         };
//     }
// }