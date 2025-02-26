use crate::board::{Board, Cell, SIZE};
use macroquad::prelude::*;

const CELL_SIZE: f32 = 50.0;

pub fn draw_board(board: &Board) {
    clear_background(LIGHTGRAY);

    for y in 0..SIZE {
        for x in 0..SIZE {
            draw_rectangle(
                x as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                DARKGREEN,
            );
            draw_rectangle_lines(
                x as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                2.0,
                BLACK,
            );
            match board.get_cell(y, x) {
                Cell::Black => draw_circle(
                    x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    CELL_SIZE / 2.5,
                    BLACK,
                ),
                Cell::White => draw_circle(
                    x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                    CELL_SIZE / 2.5,
                    WHITE,
                ),
                _ => {}
            }
        }
    }
}
