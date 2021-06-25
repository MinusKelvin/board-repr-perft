use common::{Piece, PieceLocation};

pub mod common;

mod naive;
mod naive_col_heights;
mod row_bits;
mod row_bits_col_heights;
mod column_bits;
mod column_bits_pext;

pub use naive::Naive;
pub use naive_col_heights::NaiveColHeights;
pub use row_bits::RowBits;
pub use row_bits_col_heights::RowBitsColHeights;
pub use column_bits::ColBits;
pub use column_bits_pext::ColBitsPext;

#[cfg(test)]
#[test]
fn check_same() {
    use rand::prelude::*;

    let mut piece_sequence = vec![];
    for _ in 0..1000 {
        piece_sequence.push(match thread_rng().gen_range(0..7) {
            0 => common::Piece::I,
            1 => common::Piece::O,
            2 => common::Piece::T,
            3 => common::Piece::L,
            4 => common::Piece::J,
            5 => common::Piece::S,
            6 => common::Piece::Z,
            _ => unreachable!(),
        })
    }
    dbg!(&piece_sequence);

    let results = vec![
        Naive::simulate(&piece_sequence).fumenize(),
        NaiveColHeights::simulate(&piece_sequence).fumenize(),
        RowBits::simulate(&piece_sequence).fumenize(),
        RowBitsColHeights::simulate(&piece_sequence).fumenize(),
        ColBits::simulate(&piece_sequence).fumenize(),
        ColBitsPext::simulate(&piece_sequence).fumenize(),
    ];

    dbg!(results.iter().map(|f| f.encode()).collect::<Vec<_>>());

    for i in 1..results.len() {
        assert!(results[0] == results[i]);
    }
}

pub trait Implementation {
    type Board: BoardImpl;

    const NAME: &'static str;

    fn suggest(board: &Self::Board, piece: Piece) -> Option<PieceLocation>;

    fn simulate(pieces: &[Piece]) -> Self::Board {
        let mut board = Self::Board::new();
        for &p in pieces {
            if let Some(placement) = Self::suggest(&board, p) {
                board.place(placement);
                board.collapse_lines();
            }
        }
        board
    }
}

pub trait BoardImpl {
    fn new() -> Self;
    fn place(&mut self, placement: PieceLocation);
    fn collapse_lines(&mut self) -> i32;
}
