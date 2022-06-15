// mod tests_single {

//     use multithread_minimax::example::four_by_four::*;
//     use multithread_minimax::get_best_moves;
//     #[test]
//     fn win_game_x() {
//         let mut game = TTT::new('x', 'o');
//         game.board = [
//             Some('x'),
//             Some('x'),
//             Some('x'),
//             None,
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, true);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 3);
//     }

//     #[test]
//     fn win_game_o() {
//         let mut game = TTT::new('o', 'x');
//         game.board = [
//             Some('x'),
//             Some('x'),
//             Some('x'),
//             None,
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, true);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 7);
//     }

//     #[test]
//     fn prevent_win_o() {
//         let mut game = TTT::new('o', 'x');
//         game.board = [
//             Some('x'),
//             Some('x'),
//             Some('x'),
//             None,
//             Some('o'),
//             Some('o'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, true);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 3);
//     }

//     #[test]
//     fn prevent_win_x() {
//         let mut game = TTT::new('x', 'o');
//         game.board = [
//             Some('x'),
//             Some('x'),
//             None,
//             None,
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, true);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 7);
//     }

//     #[test]
//     fn minimizer_turn() {
//         let mut game = TTT::new('x', 'o');
//         game.board = [
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             None,
//             Some('x'),
//             Some('x'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, false);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 3);
//     }

//     #[test]
//     fn minimizer_turn_2() {
//         let mut game = TTT::new('o', 'x');
//         game.board = [
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             None,
//             Some('x'),
//             Some('x'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, false);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 3);
//     }

//     #[test]
//     fn minimizer_turn_3() {
//         let mut game = TTT::new('o', 'x');
//         game.board = [
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             None,
//             Some('x'),
//             Some('x'),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, false);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 3);
//     }

//     #[test]
//     fn minimizer_turn_4() {
//         let mut game = TTT::new('o', 'x');
//         game.board = [
//             Some('o'),
//             Some('o'),
//             Some('o'),
//             Some('x'),
//             None,
//             None,
//             None,
//             Some('x'),
//             None,
//             None,
//             None,
//             Some('x'),
//             None,
//             None,
//             None,
//             None
//         ];
//         let (moves, _) = get_best_moves(game, 0, false);
//         for i in 0..moves.len() {
//             println!("{:?}", moves[i]);
//         }
//         assert_eq!(moves.len(), 1);
//         assert_eq!(moves[0].game_move.to_position, 15);
//     }
// }
