mod tests {
    use multithread_minimax::example::ttt::{TTT, Move};
    use multithread_minimax::traits::game::{Board, Result};
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
        assert!(!result.is_game_over(), "first not over");
        assert_eq!(result.score(), 0, "first score {}", result.score());
        game.make_move(&Move{ player: 'x', to_position: 4 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "second not over");
        assert_eq!(result.score(), 0, "second score {}", result.score());
        game.make_move(&Move{ player: 'o', to_position: 5 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "third not over");
        assert_eq!(result.score(), 0, "third score {}", result.score());
        game.make_move(&Move{ player: 'x', to_position: 3 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "fourth not over");
        assert_eq!(result.score(), 0, "fourth score");
        game.make_move(&Move{ player: 'o', to_position: 2 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "fifth not over");
        assert_eq!(result.score(), 0, "fifth score");
        game.make_move(&Move{ player: 'x', to_position: 0 });
        result = game.evaluate();
        assert!(!result.is_game_over(), "sixth not over");
        assert_eq!(result.score(), 0, "sixth score");
        game.make_move(&Move{ player: 'o', to_position: 8 });
        result = game.evaluate();
        assert!(result.is_game_over(), "seventh over");
        assert_eq!(result.score(), -100, "seventh score");
        game.unmake_move(&Move{ player: 'o', to_position: 5 });
        game.make_move(&Move{ player: 'x', to_position: 5 });
        result = game.evaluate();
        assert!(result.is_game_over(), "eight over");
        assert_eq!(result.score(), 100, "eighth score");
        game.unmake_move(&Move{ player: 'x', to_position: 3 });
        game.make_move(&Move{ player: 'o', to_position: 3 });
        game.make_move(&Move{ player: 'x', to_position: 6 });
        game.make_move(&Move{ player: 'x', to_position: 7 });
        game.make_move(&Move{ player: 'o', to_position: 1 });
        result = game.evaluate();
        assert!(result.is_game_over(), "ninth over");
        assert_eq!(result.score(), 0, "ninth score");
    }

    #[test]
    fn special1() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'), Some('x'), None,
            Some('o'),   None,    None,
               None,     None,    None];
        let mut result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'x', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special2() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'), Some('x'), None,
            Some('o'),   None,    None,
               None,     None,    None];
        let mut result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'o', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special3() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'), Some('x'), None,
            Some('o'),   None,    None,
               None,     None,    None];
        let mut result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'x', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special4() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'), Some('x'), None,
            Some('o'),   None,    None,
               None,     None,    None];
        let mut result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
        game.make_move(&Move{player: 'o', to_position: 6});
        result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn special5() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'), Some('x'), Some('o'),
            Some('o'),   Some('o'),    Some('x'),
               None,     Some('o'),    None];
        let result = game.evaluate();
        assert!(!result.is_game_over());
        assert_eq!(result.score(), 0);
    }

    #[test]
    fn game_over() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'), Some('x'), Some('o'),
            Some('o'),   Some('o'),    Some('x'),
            Some('o'),     Some('x'),    Some('x')];
        let result = game.evaluate();
        assert!(result.is_game_over());
        assert_eq!(result.score(), 0);
    }
    
    #[test]
    fn win_game_x() {
        let mut game = TTT::new('x', 'o');
        game.board = [Some('x'), Some('x'), None, Some('o'), Some('o'), None, None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, true);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn win_game_o() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('x'), Some('x'), None, Some('o'), Some('o'), None, None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, true);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 5);
    }

    #[test]
    fn prevent_win_o() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('x'), Some('x'), None,
            Some('o'),   None,    None,
               None,     None,    None];
        let moves = multithread_minimax::get_best_moves(game, 0, true);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn prevent_win_x() {
        let mut game = TTT::new('x', 'o');
        game.board = [Some('x'), None, None, Some('o'), Some('o'), None, None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, true);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 5);
    }

    #[test]
    fn minimizer_turn() {
        let mut game = TTT::new('x', 'o');
        game.board = [Some('o'), Some('o'), None, None, None, None, None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn minimizer_turn2() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('o'), Some('o'), None, None, None, None, None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn minimizer_turn3() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('o'), Some('o'), None, None, None, None, None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn minimizer_turn4() {
        let mut game = TTT::new('o', 'x');
        game.board = [Some('o'), Some('o'), Some('x'), None, None, Some('x'), None, None, None];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 8);
    }

}
