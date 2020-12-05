extern crate piston_window;
use piston_window::types::Color;
use piston_window::*;

// use piston_window::keyboard::Key;

mod draw;
mod game;
mod snake;

use draw::make_coords_f64;
use game::Game;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Rusty the Snake!",
        (make_coords_f64(width), make_coords_f64(height)),
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut game = Game::new(width, height);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear(BACKGROUND_COLOR, g);
            game.draw(c, g);
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }
    }
}
