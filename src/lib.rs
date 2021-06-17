pub mod common;

pub mod naive;
pub mod naive_col_heights;
pub mod row_bits;
pub mod row_bits_col_heights;
pub mod column_bits;
pub mod column_bits_pext;

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
        naive::benchmark(&piece_sequence).fumenize(),
        naive_col_heights::benchmark(&piece_sequence).fumenize(),
        row_bits::benchmark(&piece_sequence).fumenize(),
        row_bits_col_heights::benchmark(&piece_sequence).fumenize(),
        column_bits::benchmark(&piece_sequence).fumenize(),
    ];

    dbg!(results.iter().map(|f| f.encode()).collect::<Vec<_>>());

    for i in 1..results.len() {
        assert!(results[0] == results[i]);
    }
}
