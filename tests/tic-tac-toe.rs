mod tests {
    use multithread_minimax::example::ttt::{TTT, Move};
    use multithread_minimax::traits::traits::{Board, Result};
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
        game.make_move(Move{ player: 'x', to_position: 4 });
        assert_eq!(game.board(), [None, None, None, None, Some('x'), None, None, None, None]);
        game.unmake_move(Move{ player: 'x', to_position: 4 });
        assert_eq!(game.board(), [None; 9]);
    }

    #[test]
    fn get_moves() {
        let mut game = TTT::new('x', 'o');
        game.make_move(Move{ player: 'x', to_position: 4 });
        let moves = game.get_valid_moves(true);
        assert_eq!(moves.len(), 8);
        assert!(moves.iter().all(|m| m.player == 'x' && m.to_position != 4));
    }

    #[test]
    fn eval() {
        let mut game = TTT::new('x', 'o');
        let mut result = game.evaluate();
        assert!(!result.is_game_over(), "first not over");
        assert_eq!(result.score(), 0.0, "first score {}", result.score());
        game.make_move(Move{ player: 'x', to_position: 4 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "second not over");
        assert_eq!(result.score(), 0.0, "second score {}", result.score());
        game.make_move(Move{ player: 'o', to_position: 5 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "third not over");
        assert_eq!(result.score(), 0.0, "third score {}", result.score());
        game.make_move(Move{ player: 'x', to_position: 3 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "fourth not over");
        assert_eq!(result.score(), 0.0, "fourth score");
        game.make_move(Move{ player: 'o', to_position: 2 });
        println!("{}", game);
        result = game.evaluate();
        assert!(!result.is_game_over(), "fifth not over");
        assert_eq!(result.score(), 0.0, "fifth score");
        game.make_move(Move{ player: 'x', to_position: 0 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "sixth not over");
        assert_eq!(result.score(), 0.0, "sixth score");
        game.make_move(Move{ player: 'o', to_position: 8 });
        result = game.evaluate();
        assert!(result.is_game_over(), "seventh over");
        assert_eq!(result.score(), -100.0, "seventh score");
        game.unmake_move(Move{ player: 'o', to_position: 5 });
        game.make_move(Move{ player: 'x', to_position: 5 });
        result = game.evaluate();
        assert!(result.is_game_over(), "eight over");
        assert_eq!(result.score(), 100.0, "eighth score");
        game.unmake_move(Move{ player: 'x', to_position: 3 });
        game.make_move(Move{ player: 'o', to_position: 3 });
        game.make_move(Move{ player: 'x', to_position: 6 });
        game.make_move(Move{ player: 'x', to_position: 7 });
        game.make_move(Move{ player: 'o', to_position: 1 });
        result = game.evaluate();
        assert!(result.is_game_over(), "ninth over");
        assert_eq!(result.score(), 0.0, "ninth score");
    }
}
