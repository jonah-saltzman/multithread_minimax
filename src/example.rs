pub mod tic_tac_toe_4x4 {

    use crate::{Board, Result};
    use std::fmt::{self, Display};

    const WIN_CONDITIONS: [[usize; 4]; 10] = [
        [0, 1, 2, 3],
        [4, 5, 6, 7],
        [8, 9, 10, 11],
        [12, 13, 14, 15],
        [0, 4, 8, 12],
        [1, 5, 9, 13],
        [2, 6, 10, 14],
        [3, 7, 11, 15],
        [0, 5, 10, 15],
        [3, 6, 9, 12]
    ];

    #[derive(Debug, Clone, Copy)]
    pub struct TTT {
        maximizer: char,
        minimizer: char,
        pub board: [Option<char>; 16],
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Move {
        pub player: char,
        pub to_position: usize,
    }

    #[derive(Debug)]
    pub struct TttResult {
        over: bool,
        score: i64,
    }

    impl Result for TttResult {
        fn is_over(&self) -> bool {
            self.over
        }
        fn score(&self) -> i64 {
            self.score
        }
    }

    impl TTT {
        pub fn new(maximizer: char, minimizer: char) -> TTT {
            TTT {
                maximizer,
                minimizer,
                board: [None; 16],
            }
        }

        pub fn board(&self) -> [Option<char>; 16] {
            self.board
        }

        pub fn maximizer(&self) -> char {
            self.maximizer
        }

        pub fn minimizer(&self) -> char {
            self.minimizer
        }
    }

    impl Display for TTT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("\n")?;
            for i in 0..=3 {
                for j in i * 4..i * 4 + 4 {
                    let c = if let Some(p) = self.board[j] {
                        p
                    } else {
                        char::from_digit(j as u32, 10).unwrap()
                    };
                    f.write_fmt(format_args!("{} ", c))?;
                }
                if i != 3 {
                    f.write_str("\n")?;
                }
            }
            Ok(())
        }
    }

    impl Board for TTT {
        type Move = Move;
        type Result = TttResult;

        fn get_valid_moves(&self, is_maximizer: bool) -> Vec<Self::Move> {
            let mut moves: Vec<Self::Move> = Vec::new();
            let player = if is_maximizer {
                self.maximizer()
            } else {
                self.minimizer()
            };
            for (i, space) in self.board.iter().enumerate() {
                if space.is_none() {
                    moves.push(Move {
                        player,
                        to_position: i,
                    })
                }
            }
            moves
        }

        fn make_move(&mut self, valid_move: &Self::Move) {
            self.board[valid_move.to_position] = Some(valid_move.player);
        }

        fn unmake_move(&mut self, valid_move: &Self::Move) {
            self.board[valid_move.to_position] = None;
        }

        fn evaluate(&self) -> Self::Result {
            let mut full = true;
            for (i, cond) in WIN_CONDITIONS.iter().enumerate() {
                let winner: Option<char> = self.board[cond[0]];
                let mut win = true;
                if let Some(winner) = winner {
                    for i in cond.iter().skip(1) {
                        if let Some(other_player) = self.board[*i] {
                            if winner != other_player {
                                win = false;
                            }
                        } else {
                            win = false;
                            full = false;
                        }
                    }
                    if win {
                        return TttResult {
                            over: true,
                            score: if winner == self.maximizer() {
                                100
                            } else {
                                -100
                            },
                        };
                    }
                } else {
                    full = false;
                }
                if full && i / cond.len() == 1 {
                    return TttResult {
                        over: true,
                        score: 0,
                    };
                }
            }
            TttResult {
                over: false,
                score: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::tic_tac_toe_4x4::{Move, TTT};
    use crate::{Board, Result};

    #[test]
    fn new_game() {
        let game = TTT::new('x', 'o');
        assert_eq!(game.board(), [None; 16]);
        assert_eq!(game.maximizer(), 'x');
        assert_eq!(game.minimizer(), 'o');
    }

    #[test]
    fn moves() {
        let mut game = TTT::new('x', 'o');
        game.make_move(&Move {
            player: 'x',
            to_position: 4,
        });
        assert_eq!(
            game.board(),
            [None, None, None, None, Some('x'), None, None, None, None, None, None, None,  None, None, None, None]
        );
        game.unmake_move(&Move {
            player: 'x',
            to_position: 4,
        });
        assert_eq!(game.board(), [None; 16]);
    }

    #[test]
    fn get_moves() {
        let mut game = TTT::new('x', 'o');
        game.make_move(&Move {
            player: 'x',
            to_position: 4,
        });
        let moves = game.get_valid_moves(true);
        assert_eq!(moves.len(), 15);
        assert!(moves.iter().all(|m| m.player == 'x' && m.to_position != 4));
    }

    #[test]
    fn eval() {
        let mut game = TTT::new('x', 'o');
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'x',
            to_position: 4,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'o',
            to_position: 5,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'x',
            to_position: 3,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'o',
            to_position: 2,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'x',
            to_position: 0,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'o',
            to_position: 8,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'x',
            to_position: 7
        });
        game.make_move(&Move {
            player: 'x',
            to_position: 11
        });
        game.make_move(&Move {
            player: 'x',
            to_position: 15
        });
        result = game.evaluate();
        assert!(result.is_over());
        assert_eq!(result.score(), 100);
        game.unmake_move(&Move {
            player: 'x',
            to_position: 15
        });
        game.make_move(&Move {
            player: 'o',
            to_position: 6
        });
        game.make_move(&Move {
            player: 'o',
            to_position: 10
        });
        game.make_move(&Move {
            player: 'o',
            to_position: 14
        });
        result = game.evaluate();
        assert!(result.is_over());
        assert_eq!(result.score(), -100);
    }

    #[test]
    fn special1() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            None,
            None,
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
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'x',
            to_position: 6,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special2() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            None,
            None,
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
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'o',
            to_position: 6,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special3() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            None,
            None,
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
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'x',
            to_position: 6,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special4() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            None,
            None,
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
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move {
            player: 'o',
            to_position: 6,
        });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special5() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'),
            Some('x'),
            Some('x'),
            Some('o'),
            Some('o'),
            Some('o'),
            Some('o'),
            Some('x'),
            None,
            Some('o'),
            None,
            None,
            None,
            Some('o'),
            None,
            None
        ];
        let result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special6() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'),
            Some('x'),
            Some('x'),
            Some('o'),
            Some('o'),
            Some('o'),
            Some('o'),
            Some('x'),
            Some('x'),
            Some('x'),
            Some('x'),
            Some('o'),
            None,
            Some('o'),
            None,
            None
        ];
        let result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn game_over() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'),
            Some('x'),
            Some('x'),
            Some('o'),
            Some('o'),
            Some('o'),
            Some('o'),
            Some('x'),
            Some('x'),
            Some('x'),
            Some('x'),
            Some('o'),
            Some('x'),
            Some('o'),
            Some('x'),
            Some('x')
        ];
        let result = game.evaluate();
        println!("result: {:?}", result);
        assert!(result.is_over());
        assert_eq!(result.score(), 0);
    }
}
