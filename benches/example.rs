
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multithread_minimax::*;
use multithread_minimax::example::tic_tac_toe_4x4::*;

fn single_threaded(c: &mut Criterion) {
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
    c.bench_function("single threaded", |b| b.iter(|| println!("{:?}", get_best_moves(black_box(game), 0, false))));
}

fn multi_threaded(c: &mut Criterion) {
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
    c.bench_function("multi threaded", |b| b.iter(|| get_best_moves_multi(black_box(game), 0, true, 0)));
}

criterion_group!(benches, single_threaded, multi_threaded);
criterion_main!(benches);
