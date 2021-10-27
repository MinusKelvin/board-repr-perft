use crate::{BoardImpl, Implementation, common::*};

pub struct RowBits;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    rows: [u16; 40],
}

const FILLED: u16 = (1 << 10) - 1;

impl BoardImpl for Board {
    fn new() -> Self {
        Board { rows: [0; 40] }
    }

    fn place(&mut self, piece: PieceLocation) {
        for &(x, y) in &piece.cells() {
            self.rows[y as usize] |= 1 << x;
        }
    }

    fn collapse_lines(&mut self) -> i32 {
        let mut current = 0;
        for i in 0..40 {
            if self.rows[i] == FILLED {
                continue;
            }
            if self.rows[current] == 0 {
                return (i - current) as i32;
            }
            self.rows[current] = self.rows[i];
            current += 1;
        }
        for i in current..40 {
            self.rows[i] = 0;
        }
        40 - current as i32
    }
}

impl Board {
    fn get(&self, x: i8, y: i8) -> bool {
        self.rows[y as usize] & 1 << x != 0
    }

    #[cfg(test)]
    pub fn fumenize(self) -> fumen::Fumen {
        let mut fumen = fumen::Fumen::default();
        let page = fumen.add_page();
        for y in 0..23 {
            for x in 0..10 {
                if self.get(x as i8, y as i8) {
                    page.field[y][x] = fumen::CellColor::Grey;
                }
            }
        }
        fumen
    }
}

impl Implementation for RowBits {
    type Board = Board;

    const NAME: &'static str = "row bits";

fn suggest(board: &Board, piece: Piece) -> Option<PieceLocation> {
    let mut best = None;

    for &rotation in piece.sensible_rotations() {
        for x in 0..10 {
            let mut piece = PieceLocation {
                piece,
                rotation,
                x,
                y: 37,
            };

            if blocked(board, piece) {
                continue;
            }

            while !blocked(board, piece) {
                piece.y -= 1;
            }
            piece.y += 1;

            let mut board = *board;
            board.place(piece);

            let piece_cells_eliminated = piece
                .cells()
                .iter()
                .filter(|&&(_, y)| board.rows[y as usize] == FILLED)
                .count() as i32;

            let lines_cleared = board.collapse_lines();

            let mut low = 40;
            let mut high = 0;
            for &(_, y) in &piece.cells() {
                low = low.min(y);
                high = high.max(y);
            }

            let landing_height = low as i32 + high as i32;
            let eroded_piece_cells_metric = lines_cleared * piece_cells_eliminated;
            let row_transitions = row_transitions(&board);
            let column_transitions = column_transitions(&board);
            let buried_holes = buried_holes(&board);
            let wells = wells(&board);

            let score = 2 * eroded_piece_cells_metric
                - landing_height
                - 2 * row_transitions
                - 2 * column_transitions
                - 8 * buried_holes
                - 2 * wells;

            match best {
                None => best = Some((piece, score)),
                Some((_, s)) => {
                    if score > s {
                        best = Some((piece, score))
                    }
                }
            }
        }
    }

    best.map(|(p, _)| p)
}
}

fn blocked(board: &Board, piece: PieceLocation) -> bool {
    for &(x, y) in &piece.cells() {
        if x < 0 || x >= 10 || y < 0 || y >= 40 {
            return true;
        }
        if board.get(x, y) {
            return true;
        }
    }
    false
}

fn row_transitions(board: &Board) -> i32 {
    board
        .rows
        .iter()
        .map(|&row| {
            let row = row | !FILLED;
            let transitions = row ^ (row << 1 | 1);
            transitions.count_ones() as i32
        })
        .sum()
}

fn column_transitions(board: &Board) -> i32 {
    let mut count = 0;
    let mut previous = FILLED;
    for &row in &board.rows {
        count += (row ^ previous).count_ones();
        previous = row;
        if row == 0 {
            break;
        }
    }
    count as i32
}

fn buried_holes(board: &Board) -> i32 {
    let mut count = 0;
    let mut is_column_covered = 0;
    for row in board.rows.iter().rev() {
        count += (is_column_covered & !row).count_ones();
        is_column_covered |= row;
    }
    count as i32
}

fn wells(board: &Board) -> i32 {
    let mut score = 0;
    for y in 0..40 {
        let row = board.rows[y];
        // Locate the well cells in this row
        let mut well_cells = (row >> 1 | 1 << 9) & !row & (row << 1 | 1);
        score += well_cells.count_ones();
        for j in (0..y).rev() {
            // Mask off the well cells that hit the ground
            well_cells &= !board.rows[j];
            if well_cells == 0 {
                break;
            }
            score += well_cells.count_ones();
        }
        if row == 0 {
            break;
        }
    }
    score as i32
}
