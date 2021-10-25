use board_repr_perft::common::Piece;
use board_repr_perft::common::PieceLocation;
use board_repr_perft::*;
use criterion::measurement::WallTime;
use criterion::*;
use rand::prelude::*;

fn dellacherie(c: &mut Criterion) {
    fn bench<I: Implementation>(group: &mut BenchmarkGroup<WallTime>, pieces: &[Piece]) {
        group.bench_function(I::NAME, |b| b.iter(|| I::simulate(pieces)));
    }

    let piece_sequence = gen_seq(1000);

    let mut group = c.benchmark_group("dellacherie");

    bench::<Naive>(&mut group, &piece_sequence);
    bench::<NaiveColHeights>(&mut group, &piece_sequence);
    bench::<RowBits>(&mut group, &piece_sequence);
    bench::<RowBitsColHeights>(&mut group, &piece_sequence);
    bench::<ColBits>(&mut group, &piece_sequence);
    bench::<ColBitsPext>(&mut group, &piece_sequence);
}

fn advance(c: &mut Criterion) {
    let mut board = <ColBits as Implementation>::Board::new();
    let mut placements = Vec::with_capacity(10000);
    for p in gen_seq(10000) {
        if let Some(placement) = <ColBits as Implementation>::suggest(&board, p) {
            board.place(placement);
            board.collapse_lines();
            placements.push(placement);
        }
    }

    fn bench<I: Implementation>(group: &mut BenchmarkGroup<WallTime>, places: &[PieceLocation]) {
        group.bench_function(I::NAME, |b| {
            b.iter(|| {
                let mut board = I::Board::new();
                for &placement in places {
                    board.place(placement);
                    board.collapse_lines();
                }
                board
            })
        });
    }

    let mut group = c.benchmark_group("advance");

    bench::<Naive>(&mut group, &placements);
    bench::<NaiveColHeights>(&mut group, &placements);
    bench::<RowBits>(&mut group, &placements);
    bench::<RowBitsColHeights>(&mut group, &placements);
    bench::<ColBits>(&mut group, &placements);
    bench::<ColBitsPext>(&mut group, &placements);
}

criterion_group! {
    name = benchmarks;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(30));
    targets = dellacherie, advance
}

criterion_main!(benchmarks);

fn gen_seq(n: usize) -> Vec<Piece> {
    let mut rng = rand_pcg::Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);

    let mut piece_sequence = Vec::with_capacity(n);
    for _ in 0..n {
        piece_sequence.push(match rng.gen_range(0..7) {
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
    piece_sequence
}
