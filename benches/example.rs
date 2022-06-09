#[macro_use]
extern crate bencher;
extern crate multithread_minimax;

use bencher::Bencher;
use multithread_minimax::*;
use multithread_minimax::example::tic_tac_toe_3x3::*;

#[cfg(feature = "single_threaded")]
fn single_thread_no_mutex(bench: &mut Bencher) {
    let board = TTT::new('x', 'o');
    bench.iter(|| {
        for _ in 0..10 {
            get_best_moves(board, 0, true, false);
        }
    })
}
#[cfg(feature = "single_threaded")]
fn single_thread_mutex(bench: &mut Bencher) {
    let board = TTT::new('x', 'o');
    bench.iter(|| {
        for _ in 0..10 {
            get_best_moves(board, 0, true, true);
        }
    })
}

#[cfg(feature = "single_threaded")]
benchmark_group!(benches, single_thread_no_mutex, single_thread_mutex);
#[cfg(feature = "single_threaded")]
benchmark_main!(benches);