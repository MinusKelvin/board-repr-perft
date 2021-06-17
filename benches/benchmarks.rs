use board_repr_perft::common::Piece;
use board_repr_perft::*;
use criterion::*;
use rand::prelude::*;

fn bench(c: &mut Criterion) {
    let mut piece_sequence = vec![];
    for _ in 0..1000 {
        piece_sequence.push(match thread_rng().gen_range(0..7) {
            0 => Piece::I,
            1 => Piece::O,
            2 => Piece::T,
            3 => Piece::L,
            4 => Piece::J,
            5 => Piece::S,
            6 => Piece::Z,
            _ => unreachable!(),
        })
    }

    let mut group = c.benchmark_group("dellacherie");
    group.bench_function("naive", |b| b.iter(|| naive::benchmark(&piece_sequence)));
    group.bench_function("naive + col heights", |b| {
        b.iter(|| naive_col_heights::benchmark(&piece_sequence))
    });
    group.bench_function("row bits", |b| {
        b.iter(|| row_bits::benchmark(&piece_sequence))
    });
    group.bench_function("row bits + col heights", |b| {
        b.iter(|| row_bits_col_heights::benchmark(&piece_sequence))
    });
    group.bench_function("col bits", |b| {
        b.iter(|| column_bits::benchmark(&piece_sequence))
    });
    group.bench_function("col bits pext", |b| {
        b.iter(|| column_bits_pext::benchmark(&piece_sequence))
    });
}

criterion_group!(benchmarks, bench,);

criterion_main!(benchmarks);
