mod board;
mod game;
mod renderer;

use board::{Board, Cell};
use game::{apply_move, is_valid_move};
use macroquad::prelude::*;
use renderer::draw_board;

#[macroquad::main("Othello")]
async fn main() {
    let mut board = Board::new();
    let mut current_player = Cell::Black;

    loop {
        draw_board(&board);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let x = (mouse_x / 50.0) as usize;
            let y = (mouse_y / 50.0) as usize;
            if x < 8 && y < 8 && is_valid_move(&board, y, x, current_player) {
                apply_move(&mut board, y, x, current_player);
                current_player = match current_player {
                    Cell::Black => Cell::White,
                    Cell::White => Cell::Black,
                    _ => unreachable!(),
                };
            }
        }
        next_frame().await;
    }
}
