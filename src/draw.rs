use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn make_coords_f64(game_coords: i32) -> f64 {
    (game_coords as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x_pos: i32, y_pos: i32, con: Context, g: &mut G2d) {
    rectangle(
        color,
        [
            make_coords_f64(x_pos),
            make_coords_f64(y_pos),
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x_pos: i32,
    y_pos: i32,
    width: i32,
    height: i32,
    con: Context,
    g: &mut G2d,
) {
    rectangle(
        color,
        [
            make_coords_f64(x_pos),
            make_coords_f64(y_pos),
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
