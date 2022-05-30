
pub mod ttt {
    use crate::traits::traits::{Board, Result};
    use std::fmt::{self, Display};

    const WIN_CONDITIONS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];

    #[derive(Debug)]
    pub struct TTT {
        maximizer: char,
        minimizer: char,
        board: [Option<char>; 9]
    }

    #[derive(Debug)]
    pub struct Move {
        pub player: char,
        pub to_position: usize
    }

    #[derive(Debug)]
    pub struct TttResult {
        over: bool,
        score: f64
    }

    impl Result for TttResult {
        fn is_game_over(&self) -> bool {
            self.over
        }
        fn score(&self) -> f64 {
            self.score
        }
    }

    impl TTT {
        pub fn new(maximizer: char, minimizer: char) -> TTT {
            TTT { maximizer, minimizer, board: [None; 9] }
        }

        pub fn board(&self) -> [Option<char>; 9] {
            self.board
        }
    }

    impl Display for TTT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut r: fmt::Result;
            r = f.write_str("\n");
            if let Err(_) = r { return r }
            for i in 0..=2 {
                for j in (i * 3)..(i * 3 + 3) {
                    let c = if let Some(p) = self.board[j] { p } else { char::from_digit(j as u32, 10).unwrap() };
                    r = f.write_fmt(format_args!("{} ", c));
                    if let Err(_) = r { return r }
                }
                r = f.write_str("\n");
                if let Err(_) = r { return r }
            }
            r
        }
    }

    impl Board for TTT {
        type Player = char;
        type Move = Move;
        type Result = TttResult;

        fn maximizer(&self) -> Self::Player {
            self.maximizer
        }

        fn minimizer(&self) -> Self::Player {
            self.minimizer
        }

        fn get_valid_moves(&self, is_maximizer: bool) -> Vec<Self::Move> {
            let mut moves: Vec<Self::Move> = Vec::new();
            let player = if is_maximizer { self.maximizer() } else { self.minimizer() };
            for (i, space) in self.board.iter().enumerate() {
                if let None = space {
                    moves.push(Move { player , to_position: i })
                }
            }
            moves
        }

        fn make_move(&mut self, valid_move: Self::Move) -> () {
            self.board[valid_move.to_position] = Some(valid_move.player);
        }

        fn unmake_move(&mut self, valid_move: Self::Move) -> () {
            self.board[valid_move.to_position] = None;
        }

        fn evaluate(&self) -> Self::Result {
            let mut full = true;
            for (i, cond) in WIN_CONDITIONS.iter().enumerate() {
                let winner: Option<char> = self.board[cond[0]];
                let mut win = true;
                if let Some(winner) = winner {
                    for i in 1..3 {
                        if let Some(other_player) = self.board[cond[i]] {
                            if winner != other_player {
                                win = false;
                            }
                        } else {
                            win = false;
                            full = false;
                        }
                    }
                    if win {
                        return TttResult{
                            over: true,
                            score: if winner == self.maximizer() { 100.0 } else { -100.0 }
                        }
                    }
                } else {
                    full = false;
                }
                if full && i == 2 {
                    return TttResult { over: true, score: 0.0 }
                }
            }
            TttResult{ over: false, score: 0.0 }
        }
    }

}