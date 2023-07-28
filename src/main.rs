extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.05, 0.1, 0.16, 1.0];

fn main() {
    let width = 30;
    let height = 30;

    // function to initiate the new window
    let mut window: PistonWindow =
        WindowSettings::new("RSnake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // function to create a new game
    let mut game = Game::new(width, height);

    // continously listen for the event and render the window for the game updates continously
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}