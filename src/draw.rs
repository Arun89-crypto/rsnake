use piston_window::types::Color;
use piston_window::{rectangle, text, Context, G2d, Glyphs, TextureSettings};

// Constant block size
const BLOCK_SIZE: f64 = 25.0;

// public function
// function to convert a game coordinate into f64 type and multiply it by the blocksize
pub fn to_coord(game_coord: i32) -> f64 {
    return (game_coord as f64) * BLOCK_SIZE;
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    return to_coord(game_coord) as u32;
}

// public function
// function to draw the block at a particular coordinate
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        // color
        color,
        // x, y, width, height
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        // transform context
        con.transform,
        // graphics buffer
        g,
    );
}

// public function
// function to draw the main rectangular screen
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        // color
        color,
        // x, y, width, height
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        // transform context
        con.transform,
        // graphics buffer
        g,
    );
}

// pub fn draw_test(color: Color, font_size: u32, content: &str, con: &Context, g: &mut G2d) {
//     let mut glyphs = Glyphs::new(
//         "assets/FiraSans-Regular.ttf",
//         con.window.factory.clone(),
//         TextureSettings::new(),
//     )
//     .unwrap();

//     text(color, font_size, content, &mut glyphs, con.transform, g);
// }
