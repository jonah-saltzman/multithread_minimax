
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multithread_minimax::*;
use multithread_minimax::example::four_by_four::*;

fn single_threaded(c: &mut Criterion) {
    let game = get_board();
    c.bench_function("single threaded", |b| b.iter(|| get_best_moves(black_box(game), 0, false)));
}

fn multi_threaded(c: &mut Criterion) {
    let game = get_board();
    c.bench_function("multi threaded", |b| b.iter(|| get_best_moves_multi(black_box(game), 0, true, 0)));
}

fn get_board() -> TTT {
    let mut game = TTT::new('x', 'o');
    game.board = [
        Some('o'),
        Some('o'),
        None,
        None,
        Some('x'),
        Some('x'),
        None,
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
    game
}

criterion_group!(benches, single_threaded, multi_threaded);
criterion_main!(benches);
