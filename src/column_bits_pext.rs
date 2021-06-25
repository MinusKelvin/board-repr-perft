use crate::{common::*, BoardImpl, Implementation};

pub struct ColBitsPext;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    columns: [u64; 10],
}

impl BoardImpl for Board {
    fn new() -> Self {
        Board { columns: [0; 10] }
    }

    fn place(&mut self, piece: PieceLocation) {
        for &(x, y) in &piece.cells() {
            self.columns[x as usize] |= 1 << y;
        }
    }

    fn collapse_lines(&mut self) -> i32 {
        let line_clear_mask = self.line_clear_mask();
        if line_clear_mask == 0 {
            return 0;
        }

        for c in &mut self.columns {
            *c = unsafe { std::arch::x86_64::_pext_u64(*c, !line_clear_mask) };
        }
        line_clear_mask.count_ones() as i32
    }
}

impl Board {
    fn get(&self, x: i8, y: i8) -> bool {
        self.columns[x as usize] & 1 << y != 0
    }

    fn column_height(&self, x: i8) -> i8 {
        64 - self.columns[x as usize].leading_zeros() as i8
    }

    fn distance_to_ground(self, x: i8, y: i8) -> i8 {
        (!self.columns[x as usize] << (63 - y)).leading_ones() as i8
    }

    fn line_clear_mask(&self) -> u64 {
        self.columns.iter().fold(!0, |a, b| a & b)
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

impl Implementation for ColBitsPext {
    type Board = Board;

    const NAME: &'static str = "col bits pext";

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

                    piece.y = piece.y.max(board.column_height(x) - y);
                }

                let mut board = *board;
                board.place(piece);

                let line_clear_mask = board.line_clear_mask();
                let piece_cells_eliminated = piece
                    .cells()
                    .iter()
                    .filter(|&&(_, y)| line_clear_mask & 1 << y != 0)
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
    let left_side = board.columns[0].count_zeros() as i32;
    let right_side = board.columns[9].count_zeros() as i32;
    board
        .columns
        .windows(2)
        .map(|cs| (cs[0] ^ cs[1]).count_ones() as i32)
        .sum::<i32>()
        + left_side
        + right_side
}

fn column_transitions(board: &Board) -> i32 {
    board
        .columns
        .iter()
        .map(|&c| (c ^ (c << 1 | 1)).count_ones() as i32)
        .sum()
}

fn buried_holes(board: &Board) -> i32 {
    board
        .columns
        .iter()
        .map(|&c| {
            let covered_mask = (1 << (64 - c.leading_zeros())) - 1;
            (!c & covered_mask).count_ones() as i32
        })
        .sum()
}

fn wells(board: &Board) -> i32 {
    let mut cumulative_wells = 0;
    for x in 0..10 {
        let left = if x == 0 { !0 } else { board.columns[x - 1] };
        let right = if x == 9 { !0 } else { board.columns[x + 1] };

        let mut wells = left & right & !board.columns[x];

        while wells != 0 {
            let y = wells.trailing_zeros();
            cumulative_wells += board.distance_to_ground(x as i8, y as i8) as i32;
            wells &= !(1 << y);
        }
    }
    cumulative_wells
}
