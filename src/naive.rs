use crate::{common::*, BoardImpl, Implementation};

pub struct Naive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    cells: [[bool; 10]; 40],
}

impl BoardImpl for Board {
    fn new() -> Self {
        Board {
            cells: [[false; 10]; 40],
        }
    }

    fn place(&mut self, piece: PieceLocation) {
        for &(x, y) in &piece.cells() {
            self.cells[y as usize][x as usize] = true;
        }
    }

    fn collapse_lines(&mut self) -> i32 {
        let mut current = 0;
        for i in 0..40 {
            if self.cells[i] == [true; 10] {
                continue;
            }
            if self.cells[current] == [false; 10] {
                return (i - current) as i32;
            }
            self.cells[current] = self.cells[i];
            current += 1;
        }
        for i in current..40 {
            self.cells[i] = [false; 10];
        }
        40 - current as i32
    }
}

impl Board {
    fn get(&self, x: i8, y: i8) -> bool {
        self.cells[y as usize][x as usize]
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

impl Implementation for Naive {
    type Board = Board;

    const NAME: &'static str = "naive";

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
                    .filter(|&&(_, y)| board.cells[y as usize] == [true; 10])
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
        .cells
        .iter()
        .map(|row| {
            let mut previous = true;
            let mut count = 0;
            for &cell in row {
                if cell != previous {
                    count += 1;
                }
                previous = cell;
            }
            if !previous {
                count += 1;
            }
            count
        })
        .sum()
}

fn column_transitions(board: &Board) -> i32 {
    let mut count = 0;
    let mut previous = [true; 10];
    for row in &board.cells {
        count += (0..10).filter(|&x| row[x] != previous[x]).count();
        previous = *row;
        if *row == [false; 10] {
            break;
        }
    }
    count as i32
}

fn buried_holes(board: &Board) -> i32 {
    let mut count = 0;
    let mut depths = [0; 10];
    for row in board.cells.iter() {
        for x in 0..10 {
            if row[x] {
                count += depths[x];
                depths[x] = 0;
            } else {
                depths[x] += 1;
            }
        }
        if *row == [false; 10] {
            break;
        }
    }
    count
}

fn wells(board: &Board) -> i32 {
    let mut score = 0;
    let mut depths = [0; 10];
    for y in 0..40 {
        let mut all_empty = true;
        for x in 0..10 {
            if board.get(x, y) {
                depths[x as usize] = 0;
                all_empty = false;
            } else {
                depths[x as usize] += 1;
                let left = x == 0 || board.get(x - 1, y);
                let right = x == 9 || board.get(x + 1, y);
                if left && right {
                    score += depths[x as usize];
                }
            }
        }
        if all_empty {
            break;
        }
    }
    score
}
