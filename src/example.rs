
pub mod tic_tac_toe_3x3 {

    use crate::{Board, Result};
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

    #[derive(Debug, Clone, Copy)]
    pub struct TTT {
        maximizer: char,
        minimizer: char,
        pub board: [Option<char>; 9]
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Move {
        pub player: char,
        pub to_position: usize
    }

    #[derive(Debug)]
    pub struct TttResult {
        over: bool,
        score: i64
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
            TTT { maximizer, minimizer, board: [None; 9] }
        }

        pub fn board(&self) -> [Option<char>; 9] {
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
            for i in 0..=2 {
                for j in i * 3..i * 3 + 3 {
                    let c = if let Some(p) = self.board[j]
                        { p }
                    else
                        { char::from_digit(j as u32, 10).unwrap() };
                    f.write_fmt(format_args!("{} ", c))?;
                }
                if i != 2 {
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
            let player = if is_maximizer { self.maximizer() } else { self.minimizer() };
            for (i, space) in self.board.iter().enumerate() {
                if space.is_none() {
                    moves.push(Move { player , to_position: i })
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
                        return TttResult{
                            over: true,
                            score: if winner == self.maximizer() { 100 } else { -100 }
                        }
                    }
                } else {
                    full = false;
                }
                if full && i == 2 {
                    return TttResult { over: true, score: 0 }
                }
            }
            TttResult{ over: false, score: 0 }
        }
    }

}

#[cfg(test)]
mod tests {

    use super::tic_tac_toe_3x3::{TTT, Move};
    use crate::{Board, Result};

    #[test]
    fn new_game() {
        let game = TTT::new('x', 'o');
        assert_eq!(game.board(), [None; 9]);
        assert_eq!(game.maximizer(), 'x');
        assert_eq!(game.minimizer(), 'o');
    }

    #[test]
    fn moves() {
        let mut game = TTT::new('x', 'o');
        game.make_move(&Move{ player: 'x', to_position: 4 });
        assert_eq!(game.board(), [None, None, None, None, Some('x'), None, None, None, None]);
        game.unmake_move(&Move{ player: 'x', to_position: 4 });
        assert_eq!(game.board(), [None; 9]);
    }

    #[test]
    fn get_moves() {
        let mut game = TTT::new('x', 'o');
        game.make_move(&Move{ player: 'x', to_position: 4 });
        let moves = game.get_valid_moves(true);
        assert_eq!(moves.len(), 8);
        assert!(moves.iter().all(|m| m.player == 'x' && m.to_position != 4));
    }

    #[test]
    fn eval() {
        let mut game = TTT::new('x', 'o');
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{ player: 'x', to_position: 4 });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{ player: 'o', to_position: 5 });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{ player: 'x', to_position: 3 });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{ player: 'o', to_position: 2 });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{ player: 'x', to_position: 0 });
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{ player: 'o', to_position: 8 });
        result = game.evaluate();
        assert!(result.is_over());
        assert_eq!(result.score(), -100);
        game.unmake_move(&Move{ player: 'o', to_position: 5 });
        game.make_move(&Move{ player: 'x', to_position: 5 });
        result = game.evaluate();
        assert!(result.is_over());
        assert_eq!(result.score(), 100);
        game.unmake_move(&Move{ player: 'x', to_position: 3 });
        game.make_move(&Move{ player: 'o', to_position: 3 });
        game.make_move(&Move{ player: 'x', to_position: 6 });
        game.make_move(&Move{ player: 'x', to_position: 7 });
        game.make_move(&Move{ player: 'o', to_position: 1 });
        result = game.evaluate();
        assert!(result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special1() {
        let mut game = TTT::new('x', 'o');
        game.board = [Some('x'), Some('x'), None, Some('o'), None, None, None, None, None];
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'x', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special2() {
        let mut game = TTT::new('x', 'o');
        game.board = [Some('x'), Some('x'), None,Some('o'), None, None, None, None, None];
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'o', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special3() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('x'), Some('x'), None, Some('o'), None, None, None, None, None];
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'x', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special4() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('x'), Some('x'), None, Some('o'), None, None, None, None, None];
        let mut result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'o', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special5() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('x'), Some('x'), Some('o'), Some('o'), Some('o'), Some('x'), None, Some('o'), None];
        let result = game.evaluate();
        assert!(!result.is_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn game_over() {
        let mut game = TTT::new('o', 'x');
        game.board =
            [Some('x'), Some('x'), Some('o'), Some('o'), Some('o'), Some('x'), Some('o'), Some('x'), Some('x')];
        let result = game.evaluate();
        assert!(result.is_over());
        assert_eq!(result.score(), 0);
    }
}