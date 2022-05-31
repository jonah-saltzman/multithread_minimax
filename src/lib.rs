use std::cmp::Ordering;

pub mod example;

fn alphabeta<T: Board>(
    mut board: T,
    depth: usize,
    mut alpha: i64,
    mut beta: i64,
    is_max: bool,
) -> i64 {
    let result = board.evaluate();
    let mut score = result.score();

    if depth == 0 || result.is_over() {
        return match score.cmp(&0) {
            Ordering::Less => score - depth as i64,
            Ordering::Greater => score + depth as i64,
            Ordering::Equal => score,
        };
    }
    let moves = board.get_valid_moves(is_max);

    if is_max {
        score = i64::MIN;
        for m in moves {
            board.make_move(&m);
            score = score.max(alphabeta(board, depth - 1, alpha, beta, !is_max));
            board.unmake_move(&m);
            alpha = alpha.max(score);
            if score >= beta {
                break;
            }
        }
        score
    } else {
        score = i64::MAX;
        for m in moves {
            board.make_move(&m);
            score = score.min(alphabeta(board, depth - 1, alpha, beta, !is_max));
            board.unmake_move(&m);
            beta = beta.min(score);
            if score <= alpha {
                break;
            }
        }
        score
    }
}

struct MoveScore<T: Board> {
    game_move: <T as Board>::Move,
    score: i64,
}

/// Gets a vector of moves representing all equally good moves for the player
/// specified by the `is_maximizers_turn` argument.
pub fn get_best_moves<T: Board + Clone>(
    mut board: T,
    mut max_depth: usize,
    is_maximizers_turn: bool,
) -> Vec<<T as Board>::Move> {
    if max_depth == 0 {
        max_depth = usize::MAX
    }
    if board.evaluate().is_over() {
        return vec![];
    }

    let mut moves: Vec<MoveScore<T>> = board
        .get_valid_moves(is_maximizers_turn)
        .into_iter()
        .map(|m| {
            board.make_move(&m);
            let score = alphabeta(board, max_depth, i64::MIN, i64::MAX, !is_maximizers_turn);
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
    moves
        .into_iter()
        .filter_map(|m| {
            if m.score == high_score {
                Some(m.game_move)
            } else {
                None
            }
        })
        .collect()
}

pub trait Board: Copy {
    type Move;
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
