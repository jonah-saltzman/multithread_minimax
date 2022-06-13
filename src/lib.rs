mod pool;
pub mod example;

use std::cmp::Ordering as cmpOrdering;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::{Arc, Mutex, atomic::{AtomicI64, Ordering}};
use pool::ThreadPool;
use std::thread;


pub trait Board: Copy + Send + Display + 'static {
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
    mut max_depth: usize,
    is_maximizers_turn: bool
) -> (Vec<MoveScore<T>>, Metadata) {

    if max_depth == 0 {
        max_depth = usize::MAX
    }

    let metadata = Rc::new(Metadata::new());

    if board.evaluate().is_over() {
        return (vec![], Rc::try_unwrap(metadata).unwrap());
    }

    let mut moves: Vec<MoveScore<T>> = board
        .get_valid_moves(is_maximizers_turn)
        .into_iter()
        .map(|m| {
            board.make_move(&m);
            let metadata = Rc::clone(&metadata);
            let score = alphabeta(board, max_depth, i64::MIN, i64::MAX, !is_maximizers_turn, metadata);
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
        .collect(), Rc::try_unwrap(metadata).unwrap())
}

pub fn get_best_moves_multi<T: Board>(
    mut board: T,
    mut max_depth: usize,
    is_maximizers_turn: bool,
    threads: usize
) -> (Vec<MoveScore<T>>, Metadata) {


    if max_depth == 0 {
        max_depth = usize::MAX
    }

    let metadata = Arc::new(Metadata::new());

    if board.evaluate().is_over() {
        return (vec![], Arc::try_unwrap(metadata).unwrap());
    }
    println!("using max depth: {}", max_depth);
    let main = Arc::new(thread::current());
    let pool = ThreadPool::new(threads, main);
    let starting_moves = board.get_valid_moves(is_maximizers_turn);
    let starting_moves_len = starting_moves.len();
    let moves: Arc<Mutex<Vec<MoveScore<T>>>> = Arc::new(Mutex::new(vec![]));
    for m in starting_moves {
        board.make_move(&m);
        let metadata = Arc::clone(&metadata);
        let moves = Arc::clone(&moves);
        let alpha = Arc::new(AtomicI64::new(i64::MIN));
        let beta = Arc::new(AtomicI64::new(i64::MAX));
        pool.execute(move || { 
            let score = alphabeta_multi(
                board,
                max_depth,
                Arc::clone(&alpha),
                Arc::clone(&beta),
                !is_maximizers_turn,
                metadata
            );
            println!("{}", board);
            println!("score: {}", score);
            moves.lock().unwrap().push(MoveScore { game_move: m, score });
        });
        board.unmake_move(&m);
    }

    {
        let moves = Arc::clone(&moves);
        while moves.lock().unwrap().len() < starting_moves_len {
            thread::park()
        }
    }

    let mut moves = moves.lock().unwrap().clone();

    moves.sort_by(|a, b| {
        if is_maximizers_turn {
            b.score.partial_cmp(&a.score).unwrap()
        } else {
            a.score.partial_cmp(&b.score).unwrap()
        }
    });

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

fn alphabeta<T: Board>(
    mut board: T,
    depth: usize,
    mut alpha: i64,
    mut beta: i64,
    is_max: bool,
    metadata: Rc<Metadata>
) -> i64 {
    let result = board.evaluate();
    let mut score = result.score();
    {
        metadata.moves.fetch_add(1, Ordering::Relaxed);
    }
    if depth == 0 || result.is_over() {
        return match score.cmp(&0) {
            cmpOrdering::Less => score - depth as i64,
            cmpOrdering::Greater => score + depth as i64,
            cmpOrdering::Equal => score,
        };
    }

    let moves = board.get_valid_moves(is_max);

    if is_max {
        score = i64::MIN;
        for m in moves {
            board.make_move(&m);
            score = score.max(alphabeta(board, depth - 1, alpha, beta, !is_max, Rc::clone(&metadata)));
            board.unmake_move(&m);
            alpha = alpha.max(score);
            if score >= beta {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
        score
    } else {
        score = i64::MAX;
        for m in moves {
            board.make_move(&m);
            score = score.min(alphabeta(board, depth - 1, alpha, beta, !is_max, Rc::clone(&metadata)));
            board.unmake_move(&m);
            beta = beta.min(score);
            if score <= alpha {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
        score
    }
}

fn alphabeta_multi<T: Board>(
    mut board: T,
    depth: usize,
    alpha: Arc<AtomicI64>,
    beta: Arc<AtomicI64>,
    is_max: bool,
    metadata: Arc<Metadata>
) -> i64 {
    let result = board.evaluate();
    println!("{}", board);
    println!("score: {}", result.score());
    println!("over: {}", result.is_over());
    let mut score = result.score();
    {
        metadata.moves.fetch_add(1, Ordering::Relaxed);
    }
    if depth == 0 || result.is_over() {
        println!("returning: {}", score);
        return match score.cmp(&0) {
            cmpOrdering::Less => score - depth as i64,
            cmpOrdering::Greater => score + depth as i64,
            cmpOrdering::Equal => score,
        };
    }

    let moves = board.get_valid_moves(is_max);

    if is_max {
        score = i64::MIN;
        for m in moves {
            board.make_move(&m);
            score = score.max(alphabeta_multi(board, depth - 1, Arc::clone(&alpha), Arc::clone(&beta), !is_max, Arc::clone(&metadata)));
            board.unmake_move(&m);
            alpha.fetch_max(score, Ordering::AcqRel);
            if score >= beta.load(Ordering::Acquire) {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
        score
    } else {
        score = i64::MAX;
        for m in moves {
            board.make_move(&m);
            score = score.min(alphabeta_multi(board, depth - 1, Arc::clone(&alpha), Arc::clone(&beta), !is_max, Arc::clone(&metadata)));
            board.unmake_move(&m);
            beta.fetch_min(score, Ordering::AcqRel);
            if score <= alpha.load(Ordering::Acquire) {
                metadata.prunes.fetch_add(1, Ordering::Relaxed);
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