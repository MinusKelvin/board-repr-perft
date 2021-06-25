use crate::{common::*, BoardImpl, Implementation};

pub struct RowBitsColHeights;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    rows: [u16; 40],
    col_heights: [i8; 10],
}

const FILLED: u16 = (1 << 10) - 1;

impl BoardImpl for Board {
    fn new() -> Self {
        Board {
            rows: [0; 40],
            col_heights: [0; 10],
        }
    }

    fn place(&mut self, piece: PieceLocation) {
        for &(x, y) in &piece.cells() {
            self.rows[y as usize] |= 1 << x;
            self.col_heights[x as usize] = self.col_heights[x as usize].max(y + 1);
        }
    }

    fn collapse_lines(&mut self) -> i32 {
        let mut current = 0;
        let max_y = self.highest() as usize;
        for i in 0..max_y {
            if self.rows[i] == FILLED {
                continue;
            }
            self.rows[current] = self.rows[i];
            current += 1;
        }
        for i in current..max_y {
            self.rows[i] = 0;
        }
        let rows_cleared = max_y - current;
        for x in 0..10 {
            self.col_heights[x] -= rows_cleared as i8;
            while self.col_heights[x] > 0 && !self.get(x as i8, self.col_heights[x] - 1) {
                self.col_heights[x] -= 1;
            }
        }
        rows_cleared as i32
    }
}

impl Board {
    fn get(&self, x: i8, y: i8) -> bool {
        self.rows[y as usize] & 1 << x != 0
    }

    fn highest(&self) -> i8 {
        self.col_heights.iter().copied().max().unwrap()
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

impl Implementation for RowBitsColHeights {
    type Board = Board;

    const NAME: &'static str = "row bits + col heights";

    fn suggest(board: &Board, piece: Piece) -> Option<PieceLocation> {
        let mut best = None;

        for &rotation in piece.sensible_rotations() {
            'placement: for x in 0..10 {
                let mut piece = PieceLocation {
                    piece,
                    rotation,
                    x,
                    y: 0,
                };

                for &(x, y) in &piece.cells() {
                    if x < 0 || x >= 10 {
                        continue 'placement;
                    }

                    piece.y = piece.y.max(board.col_heights[x as usize] - y);
                }

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

fn row_transitions(board: &Board) -> i32 {
    let highest = board.highest();
    let extras = (40 - highest as i32) * 2;
    board
        .rows
        .iter()
        .take(highest as usize)
        .map(|&row| {
            let row = row | !FILLED;
            let transitions = row ^ (row << 1 | 1);
            transitions.count_ones() as i32
        })
        .sum::<i32>()
        + extras
}

fn column_transitions(board: &Board) -> i32 {
    let mut count = 0;
    let mut previous = FILLED;
    for &row in &board.rows[..board.highest() as usize + 1] {
        count += (row ^ previous).count_ones();
        previous = row;
    }
    count as i32
}

fn buried_holes(board: &Board) -> i32 {
    let mut count = 0;
    let mut is_column_covered = 0;
    for row in board.rows[..board.highest() as usize].iter().rev() {
        count += (is_column_covered & !row).count_ones();
        is_column_covered |= row;
    }
    count as i32
}

fn wells(board: &Board) -> i32 {
    let mut score = 0;
    for y in 0..board.highest() {
        for x in 0..10 {
            let left = x == 0 || board.get(x - 1, y);
            let right = x == 9 || board.get(x + 1, y);
            if left && right && !board.get(x, y) {
                // Count the number of empty cells below, including the well cell
                for y in (0..=y).rev() {
                    if board.get(x, y) {
                        break;
                    }
                    score += 1;
                }
            }
        }
    }
    score
}
