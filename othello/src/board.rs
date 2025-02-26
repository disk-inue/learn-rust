pub const SIZE: usize = 8;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

pub struct Board {
    pub cells: [[Cell; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            cells: [[Cell::Empty; SIZE]; SIZE],
        };
        board.cells[3][3] = Cell::White;
        board.cells[4][4] = Cell::White;
        board.cells[3][4] = Cell::Black;
        board.cells[4][3] = Cell::Black;
        board
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }
}
