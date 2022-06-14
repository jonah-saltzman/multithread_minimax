#[cfg(test)]
mod tests {

    use multithread_minimax::example::four_by_four::TTT;
    use multithread_minimax::get_best_moves;

    #[test]
    fn prevent_win_x_0() {
        let mut game = TTT::new('x', 'o');
        game.board = [
            Some('x'),
            Some('x'),
            None,
            None,
            Some('o'),
            Some('o'),
            Some('o'),
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
        let (moves, _) = get_best_moves(game, 0, true);
        for i in 0..moves.len() {
            println!("{:?}", moves[i]);
        }
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].game_move.to_position, 7);
    }

    // #[test]
    // fn prevent_win_x_1() {
    //     let mut game = TTT::new('x', 'o');
    //     game.board = [
    //         Some('o'),
    //         Some('o'),
    //         Some('o'),
    //         None,
    //         Some('x'),
    //         Some('x'),
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None
    //     ];
    //     let (moves, _) = get_best_moves(game, 0, true);
    //     for i in 0..moves.len() {
    //         println!("{:?}", moves[i]);
    //     }
    //     assert_eq!(moves.len(), 1);
    //     assert_eq!(moves[0].game_move.to_position, 3);
    // }
}