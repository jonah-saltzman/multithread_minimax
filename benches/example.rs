
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multithread_minimax::*;
use multithread_minimax::example::tic_tac_toe_3x3::*;

#[cfg(feature = "single_threaded")]
fn criterion_benchmark(c: &mut Criterion) {
    let mut game = TTT::new('x', 'o');
    game.board = [
        Some('x'),
        Some('x'),
        Some('x'),
        None,
        Some('o'),
        Some('o'),
        Some('o'),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ];
    c.bench_function("single threaded no mut", |b| b.iter(|| println!("{:?}", get_best_moves(black_box(game), 0, false, false))));
}

#[cfg(not(feature = "single_threaded"))]
fn criterion_benchmark(c: &mut Criterion) {
    let mut game = TTT::new('x', 'o');
    game.board = [
        Some('x'),
        Some('x'),
        Some('x'),
        None,
        Some('o'),
        Some('o'),
        Some('o'),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ];
    c.bench_function("single threaded no mut", |b| b.iter(|| get_best_moves(black_box(game), 0, true, false)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);