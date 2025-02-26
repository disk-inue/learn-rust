use crate::board::{Board, Cell, SIZE};

pub fn is_valid_move(board: &Board, row: usize, col: usize, player: Cell) -> bool {
    if board.get_cell(row, col) != Cell::Empty {
        return false;
    }
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let opponent = match player {
        Cell::Black => Cell::White,
        Cell::White => Cell::Black,
        _ => return false,
    };

    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut found_opponent = false;

        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            match board.get_cell(r as usize, c as usize) {
                cell if cell == opponent => found_opponent = true,
                cell if cell == player && found_opponent => return true,
                _ => break,
            }
            r += dr;
            c += dc;
        }
    }
    false
}

pub fn apply_move(board: &mut Board, row: usize, col: usize, player: Cell) {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    board.set_cell(row, col, player);
    let opponent = match player {
        Cell::Black => Cell::White,
        Cell::White => Cell::Black,
        _ => return,
    };

    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut to_flip = Vec::new();

        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            match board.get_cell(r as usize, c as usize) {
                cell if cell == opponent => to_flip.push((r as usize, c as usize)),
                cell if cell == player => {
                    for &(fr, fc) in &to_flip {
                        board.set_cell(fr, fc, player);
                    }
                    break;
                }
                _ => break,
            }
            r += dr;
            c += dc;
        }
    }
}
