#[cfg(test)]
mod tests {

    use multithread_minimax::example::tic_tac_toe_3x3::TTT;

    #[test]
    fn win_game_x() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            Some('o'),
            None,
            None,
            None,
            None,
        ];
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
        game.board = [
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            Some('o'),
            None,
            None,
            None,
            None,
        ];
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
            Some('x'),
            Some('x'),
            None,
            Some('o'),
            None,
            None,
            None,
            None,
            None,
        ];
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
        game.board = [
            Some('x'),
            None,
            None,
            Some('o'),
            Some('o'),
            None,
            None,
            None,
            None,
        ];
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
        game.board = [
            Some('o'),
            Some('o'),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn minimizer_turn_2() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('o'),
            Some('o'),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn minimizer_turn_3() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('o'),
            Some('o'),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 2);
    }

    #[test]
    fn minimizer_turn_4() {
        let mut game = TTT::new('o', 'x');
        game.board = [
            Some('o'),
            Some('o'),
            Some('x'),
            None,
            None,
            Some('x'),
            None,
            None,
            None,
        ];
        let moves = multithread_minimax::get_best_moves(game, 0, false);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].to_position, 8);
    }
}
