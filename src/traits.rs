pub mod traits {

    pub trait Board {

        type Player;
        type Move;
        type Result: Result;

        /// make_move should assume that valid_move will be a valid
        /// move for the current board state
        fn make_move(&mut self, valid_move: Self::Move) -> ();

        /// unmake_move should assume that made_move is a valid move
        /// that has already been made by calling [Board::make_move]
        /// with the same move
        fn unmake_move(&mut self, made_move: Self::Move) -> ();

        /// must return all valid moves for the given player. Returning
        /// invalid moves is a logic error that will cause the engine
        /// to produce invalid results
        fn get_valid_moves(&self, is_maximizer: bool) -> Vec<Self::Move>;

        /// evaluate returns a struct that implements the [Result] trait.
        /// The value returned by [Result::score] will be ignored unless
        /// [Result::is_game_over] returns true.
        fn evaluate(&self) -> Self::Result;

        /// returns the maximizing player
        fn maximizer(&self) -> Self::Player;

        /// returns the minimizing player
        fn minimizer(&self) -> Self::Player;

    }

    pub trait Result {
        /// should return true if the game is over for any reason
        /// i.e. a player has won or there is a draw
        fn is_game_over(&self) -> bool;

        /// returns the score associated with this result. The
        /// maximizing player will seek the maximum score while
        /// the minimizing player will seek the minimum score.
        fn score(&self) -> f64;
    }
}