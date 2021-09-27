use std::cmp::{max, min};

use crate::card::*;
use crate::utility::*;

// returns a vector of (card, row, column) of available moves
fn available_moves(
    board: &[[Option<Card>; 5]; 4],
    bombs: &mut [[u8; 5]; 4],
    deck: &Vec<Card>,
    player: u8,
) -> Vec<(usize, usize, usize, [Option<(usize, usize)>; 4])> {
    let mut moves: Vec<(usize, usize, usize, [Option<(usize, usize)>; 4])> = Default::default();

    // iterate through the boards for each card in deck
    for i in 0..4 {
        for j in 0..5 {
            for d in 0..deck.len() {
                if board[i][j].is_none() {
                    // fetch neighbours
                    let neighbours = Card::get_neighbours(board, i, j, deck[d].name);

                    // flag to determine whether the move has priority
                    let mut has_priority: bool = false;

                    if bombs[i][j] > 0 {
                        has_priority = false;
                    }

                    // iterate through neighbours to set priority when there are any oppenent card neighbours for that cell
                    for i in 0..4 {
                        // if there is a valid neighbouring cell
                        if let Some(n) = neighbours[i] {
                            // if there is a card in that cell
                            if let Some(neighbour) = board[n.0][n.1].as_ref() {
                                // if that card belongs to the opponent
                                if neighbour.player != player {
                                    has_priority = true;
                                    break;
                                }
                            }
                        }
                    }

                    // if move has priority, add it to the beginning of the list
                    if has_priority {
                        moves.insert(0, (d, i, j, neighbours));
                    }
                    // otherwise add it to the end of the list
                    else {
                        moves.push((d, i, j, neighbours));
                    }
                }
            }
        }
    }

    return moves;
}

// plays the best move on the board for current player
pub fn ai(
    board: &mut [[Option<Card>; 5]; 4],
    deck1: &mut Vec<Card>,
    deck2: &mut Vec<Card>,
    player: u8,
    bombs: &mut [[u8; 5]; 4],
) -> (usize, usize, usize, [Option<(usize, usize)>; 4]) {
    // init best move and scores
    let mut best_move: usize;
    let mut best_score: i8;
    let moves: Vec<(usize, usize, usize, [Option<(usize, usize)>; 4])>;

    if player == 1 {
        best_score = -120;
        best_move = 0;
        moves = available_moves(board, bombs, deck1, 1);
    } else {
        best_score = 120;
        best_move = 0;
        moves = available_moves(board, bombs, deck2, 2);
    }

    // determines maximum depth of minimax algorithm
    let max_depth: u8 = {
        match deck1.len() + deck2.len() {
            0..=8 => 8,
            9..=10 => 5,
            _ => 3,
        }
    };

    // iterate through the moves
    for m in 0..moves.len() {
        println!(" === Progress: {}/{}", m + 1, moves.len());

        // determine move
        let mut mov = moves[m];

        // make a copy of the board
        let temp_board: [[Option<Card>; 5]; 4] = copy_board(&board);

        // save card
        let temp_card = {
            if player == 1 {
                Card::copy(&deck1[mov.0])
            } else {
                Card::copy(&deck2[mov.0])
            }
        };

        let temp_bombs = bombs.clone();

        // place the card down
        Card::place_card(
            board,
            deck1,
            deck2,
            mov.0,
            (mov.1, mov.2),
            player,
            bombs,
            &mut mov.3,
        );

        // maximising player
        if player == 1 {
            // calculate score for this move
            let score = minimax(board, deck1, deck2, bombs, 2, -120, 120, max_depth);

            if score > best_score {
                println!(
                    " === New HIGH score for MAX @ {}, {} with a {:?} !! It's {}",
                    mov.1, mov.2, temp_card.name, score
                );
                // we found a better move
                best_score = score;
                best_move = m;
            }

            // put the card back
            deck1.insert(mov.0, temp_card);
        }
        // minimizing player
        else {
            // calculate score for this move
            let score = minimax(board, deck1, deck2, bombs, 1, -120, 120, max_depth);

            if score < best_score {
                println!(
                    " === New LOW score for MIN @ {}, {} with a {:?} !! It's {}",
                    mov.1, mov.2, temp_card.name, score
                );
                // we found a better move
                best_score = score;
                best_move = m;
            }

            // put the card back
            deck2.insert(mov.0, temp_card);
        }

        // revert the board
        *board = temp_board;

        // revert bombs
        *bombs = temp_bombs;
    }

    if (player == 1 && best_score == 120) || (player == 2 && best_score == -120) {
        println!("\n  Omae wa mou shindeiru");
    }

    return moves[best_move];
}

fn minimax(
    board: &mut [[Option<Card>; 5]; 4],
    deck1: &mut Vec<Card>,
    deck2: &mut Vec<Card>,
    bombs: &mut [[u8; 5]; 4],
    player: u8,
    mut alpha: i8,
    mut beta: i8,
    depth: u8,
) -> i8 {
    // if player 2 is out of cards, the game is over
    if deck2.is_empty() {
        let (p1_score, p2_score) = calc_scores(&board);
        if p1_score > p2_score {
            return 120;
        } else {
            return -120;
        }
    }

    // if we are out of depth, return static evaluation
    if depth == 0 {
        return evaluation(board);
    }

    // get all possible moves & init score
    let moves: Vec<(usize, usize, usize, [Option<Position>; 4])>;
    let mut best_score: i8;
    if player == 1 {
        moves = available_moves(board, bombs, deck1, 1);
        best_score = -120;
    } else {
        moves = available_moves(board, bombs, deck2, 2);
        best_score = 120;
    }

    // iterate through moves
    for m in 0..moves.len() {
        // determine move
        let mut mov = moves[m];

        // make a copy of the board
        let temp_board: [[Option<Card>; 5]; 4] = copy_board(&board);

        // save card
        let temp_card = {
            if player == 1 {
                Card::copy(&deck1[mov.0])
            } else {
                Card::copy(&deck2[mov.0])
            }
        };

        // make a copy of the bombs
        let temp_bombs = bombs.clone();

        Card::place_card(
            board,
            deck1,
            deck2,
            mov.0,
            (mov.1, mov.2),
            player,
            bombs,
            &mut mov.3,
        );

        if player == 1 {
            best_score = max(
                best_score,
                minimax(board, deck1, deck2, bombs, 2, alpha, beta, depth - 1),
            );

            // put the taken card back
            deck1.insert(mov.0, temp_card);
            // revert board
            *board = temp_board;
            // revert bombs
            *bombs = temp_bombs;

            if best_score >= beta {
                break;
            }

            alpha = max(alpha, best_score);
        } else {
            best_score = min(
                best_score,
                minimax(board, deck1, deck2, bombs, 1, alpha, beta, depth - 1),
            );

            // put the taken card back
            deck2.insert(mov.0, temp_card);
            // revert board
            *board = temp_board;
            // revert bombs
            *bombs = temp_bombs;

            if best_score <= alpha {
                break;
            }

            beta = min(beta, best_score);
        }
    }

    return best_score;
}
