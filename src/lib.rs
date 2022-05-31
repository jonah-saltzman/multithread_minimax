pub mod traits;
pub mod example;

use traits::game::{Board, Result};
use std::fmt::Display;

fn alphabeta<T: Board + Display>(mut board: T, depth: usize, mut alpha: i64, mut beta: i64, is_max: bool) -> i64 {
    let result = board.evaluate();
    if depth == 0 || result.is_game_over() {
        return if result.score() > 0 { result.score() + depth as i64 }
        else if result.score() < 0 { result.score() - depth as i64 }
        else { result.score() }
    }
    let moves = board.get_valid_moves(is_max);
    let mut score: i64;
    if is_max {
        score = i64::MIN;
        for m in moves {
            board.make_move(&m);
            score = score.max(alphabeta(board, depth - 1, alpha, beta, !is_max));
            board.unmake_move(&m);
            alpha = alpha.max(score);
            if score >= beta { break }
        }
        return score
    } else {
        score = i64::MAX;
        for m in moves {
            board.make_move(&m);
            score = score.min(alphabeta(board, depth - 1, alpha, beta, !is_max));
            board.unmake_move(&m);
            beta = beta.min(score);
            if score <= alpha { break }
        }
        return score
    }
}

#[derive(Debug)]
struct MoveScore<T: Board> {
    game_move: <T as Board>::Move,
    score: i64
}

/// Gets a vector of moves representing all equally good moves for the player
/// specified by the `is_maximizers_turn` argument.
pub fn get_best_moves<T: Board + Clone + Display>(
    mut board: T,
    mut max_depth: usize,
    is_maximizers_turn: bool
) -> Vec<<T as Board>::Move> {
    if max_depth == 0 { max_depth = usize::MAX }
    let pre_eval = board.evaluate();
    if pre_eval.is_game_over() { return vec![] }
    let mut moves: Vec<MoveScore<T>> = board.get_valid_moves(is_maximizers_turn).into_iter().map(|m| {
        board.make_move(&m);
        let score = alphabeta(board.clone(), max_depth, i64::MIN, i64::MAX, !is_maximizers_turn);
        board.unmake_move(&m);
        MoveScore{ game_move: m, score }
    }).collect();
    moves.sort_by(|a, b| {
        if is_maximizers_turn { b.score.partial_cmp(&a.score).unwrap() }
        else { a.score.partial_cmp(&b.score).unwrap() }
    });
    let high_score = moves[0].score;
    moves.into_iter().filter_map(|m| if m.score == high_score { Some(m.game_move) } else { None } ).collect()
}