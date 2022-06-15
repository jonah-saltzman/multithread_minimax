mod pool;
pub mod example;

use std::cmp::Ordering as cmpOrdering;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::{Arc, Mutex, atomic::{AtomicI64, Ordering}};
use pool::ThreadPool;
use std::thread;


pub trait Board: Copy + Send + Display + Debug + 'static {
    type Move: Copy + Send + Debug + 'static;
    type Result: Result;

    /// `make_move` should assume that valid_move will be a valid
    /// move for the current board state
    fn make_move(&mut self, valid_move: &Self::Move);

    /// `unmake_move` should assume that made_move is a valid move
    /// that has already been made by calling [Board::make_move]
    /// with the same move
    fn unmake_move(&mut self, made_move: &Self::Move);

    /// Must return all valid moves for the given player. Returning
    /// invalid moves is a logic error that will cause the engine
    /// to produce invalid results
    fn get_valid_moves(&self, is_maximizer: bool) -> Vec<Self::Move>;

    /// `evaluate` returns a struct that implements the [Result] trait.
    /// The value returned by [Result::score] will be ignored unless
    /// [Result::is_over] returns true OR the recursive depth
    /// has been reached.
    fn evaluate(&self) -> Self::Result;
}

pub trait Result {
    /// Should return true if the game is over for any reason
    /// i.e. a player has won or there is a draw
    fn is_over(&self) -> bool;

    /// Returns the score associated with this result. The
    /// maximizing player will seek the maximum score while
    /// the minimizing player will seek the minimum score.
    fn score(&self) -> i64;
}

// struct AlphaBeta {
//     alpha: i64,
//     beta: i64
// }

#[derive(Debug)]
pub struct Metadata {
    moves: AtomicI64,
    prunes: AtomicI64
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata { moves: AtomicI64::new(0), prunes: AtomicI64::new(0) }
    }
}

/// Gets a vector of moves representing all equally good moves for the player
/// specified by the `is_maximizers_turn` argument.
pub fn get_best_moves<T: Board>(
    mut board: T,
    mut max_depth: u16,
    is_maximizers_turn: bool,
    atomics: bool
) -> (Vec<MoveScore<T>>, Metadata) {

    if max_depth == 0 {
        max_depth = u16::MAX
    }

    let metadata = Arc::new(Metadata::new());

    if board.evaluate().is_over() {
        return (vec![], Arc::try_unwrap(metadata).unwrap());
    }

    if atomics {
        let mut moves: Vec<MoveScore<T>> = board
        .get_valid_moves(is_maximizers_turn)
        .into_iter()
        .map(|m| {
            board.make_move(&m);
            let metadata = Arc::clone(&metadata);
            let score = alphabeta_atomic(
                &mut board,
                0,
                max_depth,
                &AtomicI64::new(i64::MIN),
                &AtomicI64::new(i64::MAX),
                !is_maximizers_turn,
                metadata
            );
            board.unmake_move(&m);
            MoveScore {
                game_move: m,
                score,
            }
        })
        .collect();

        moves.sort_by(|a, b| {
            if is_maximizers_turn {
                b.score.partial_cmp(&a.score).unwrap()
            } else {
                a.score.partial_cmp(&b.score).unwrap()
            }
        });
    
        println!("final scores:");
        moves.iter().for_each(|m| {println!("{:?}", m);});
    
        let high_score = moves[0].score;
    
        (moves
            .into_iter()
            .filter_map(|m| {
                if m.score == high_score {
                    Some(m)
                } else {
                    None
                }
            })
            .collect(), Arc::try_unwrap(metadata).unwrap())
    } else {
        let mut moves: Vec<MoveScore<T>> = board
        .get_valid_moves(is_maximizers_turn)
        .into_iter()
        .map(|m| {
            board.make_move(&m);
            let metadata = Arc::clone(&metadata);
            let score = alphabeta_int(
                &mut board,
                0,
                max_depth,
                i64::MIN,
                i64::MAX,
                !is_maximizers_turn,
                metadata
            );
            board.unmake_move(&m);
            MoveScore {
                game_move: m,
                score,
            }
        })
        .collect();

        moves.sort_by(|a, b| {
            if is_maximizers_turn {
                b.score.partial_cmp(&a.score).unwrap()
            } else {
                a.score.partial_cmp(&b.score).unwrap()
            }
        });
    
        println!("final scores:");
        moves.iter().for_each(|m| {println!("{:?}", m);});
    
        let high_score = moves[0].score;
    
        (moves
            .into_iter()
            .filter_map(|m| {
                if m.score == high_score {
                    Some(m)
                } else {
                    None
                }
            })
            .collect(), Arc::try_unwrap(metadata).unwrap())
    }
    
}

fn alphabeta_atomic<T: Board>(
    board: &mut T,
    depth: u16,
    max_depth: u16,
    alpha: &AtomicI64,
    beta: &AtomicI64,
    is_max: bool,
    metadata: Arc<Metadata>
) -> i64 {
    let result = board.evaluate();
    let mut score = result.score();
    {
        metadata.moves.fetch_add(1, Ordering::Relaxed);
    }
    if depth == max_depth || result.is_over() {
        let ret = match score.cmp(&0) {
            cmpOrdering::Less => score + (depth as i64),
            cmpOrdering::Greater => score - (depth as i64),
            cmpOrdering::Equal => score,
        };
        // println!("{}", board);
        // println!("depth as i64: {}", depth as i64);
        // println!("depth: {}", depth);
        // println!("score: {}", score);
        // println!("returning: {}", ret);
        return ret
    }

    let moves = board.get_valid_moves(is_max);

    if is_max {
        score = i64::MIN;
        for m in moves {
            board.make_move(&m);
            score = score.max(alphabeta_atomic(
                board,
                depth + 1,
                max_depth,
                alpha,
                beta,
                false,
                Arc::clone(&metadata)
            ));
            board.unmake_move(&m);
            alpha.fetch_max(score, Ordering::SeqCst);
            if score >= beta.load(Ordering::SeqCst) {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                if depth < 1000 { println!("beta = {}, prune depth = {}", beta.load(Ordering::SeqCst), depth) }
                break;
            }
        }
        score
    } else {
        score = i64::MAX;
        for m in moves {
            board.make_move(&m);
            score = score.min(alphabeta_atomic(board,
                depth + 1,
                max_depth,
                alpha,
                beta,
                true,
                Arc::clone(&metadata)
            ));
            board.unmake_move(&m);
            beta.fetch_min(score, Ordering::SeqCst);
            if score <= alpha.load(Ordering::SeqCst) {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                if depth < 1000 { println!("alpha = {}, prune depth = {}", alpha.load(Ordering::SeqCst), depth) }
                break;
            }
        }
        score
    }
}

fn alphabeta_int<T: Board>(
    board: &mut T,
    depth: u16,
    max_depth: u16,
    mut alpha: i64,
    mut beta: i64,
    is_max: bool,
    metadata: Arc<Metadata>
) -> i64 {
    let result = board.evaluate();
    let mut score = result.score();
    {
        metadata.moves.fetch_add(1, Ordering::Relaxed);
    }
    if depth == max_depth || result.is_over() {
        let ret = match score.cmp(&0) {
            cmpOrdering::Less => score + (depth as i64),
            cmpOrdering::Greater => score - (depth as i64),
            cmpOrdering::Equal => score,
        };
        // println!("{}", board);
        // println!("depth as i64: {}", depth as i64);
        // println!("depth: {}", depth);
        // println!("score: {}", score);
        // println!("returning: {}", ret);
        return ret
    }

    let moves = board.get_valid_moves(is_max);

    if is_max {
        score = i64::MIN;
        for m in moves {
            board.make_move(&m);
            score = score.max(alphabeta_int(
                board,
                depth + 1,
                max_depth,
                alpha,
                beta,
                false,
                Arc::clone(&metadata)
            ));
            board.unmake_move(&m);
            alpha = alpha.max(score);
            if score >= beta {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                if depth < 1 { println!("beta = {}, prune depth = {}", beta, depth) }
                break;
            }
        }
        score
    } else {
        score = i64::MAX;
        for m in moves {
            board.make_move(&m);
            score = score.min(alphabeta_int(board,
                depth + 1,
                max_depth,
                alpha,
                beta,
                true,
                Arc::clone(&metadata)
            ));
            board.unmake_move(&m);
            beta = beta.min(score);
            if score <= alpha {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                if depth < 1 { println!("alpha = {}, prune depth = {}", alpha, depth) }
                break;
            }
        }
        score
    }
}

#[derive(Clone, Debug)]
pub struct MoveScore<T: Board> {
    pub game_move: <T as Board>::Move,
    pub score: i64,
}