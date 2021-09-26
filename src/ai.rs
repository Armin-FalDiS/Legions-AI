use std::cmp::{max, min};

use crate::card::*;
use crate::utility::*;

// returns a vector of (card, row, column) of available moves
fn available_moves(
    board: &[[Option<Card>; 5]; 4],
    bombs: &[[u8; 5]; 4],
    deck: &Vec<Card>,
    player: u8,
) -> Vec<(usize, usize, usize)> {
    let mut moves: Vec<(usize, usize, usize)> = Default::default();

    // iterate through the boards for each card in deck
    for i in 0..4 {
        for j in 0..5 {
            for d in 0..deck.len() {
                if board[i][j].is_none() {
                    // flag to determine whether the move has priority
                    let has_priority: bool = {
                        // if there are bombs in that cell, try to avoid them
                        if bombs[i][j] > 0 {
                            false
                        } else {
                            match deck[d].name {
                                // Keeper and Siren affect cards at range so having a far neighbour could be good
                                Unit::Keeper | Unit::Siren => {
                                    // fetch top neighbour
                                    let top = Card::get_far_neighbour(Direction::Top, board, i, j);
                                    let top: Option<&Card> = {
                                        if top.is_some() {
                                            board[top.unwrap()][j].as_ref()
                                        } else {
                                            None
                                        }
                                    };
                                    // fetch bottom neighbour
                                    let bottom =
                                        Card::get_far_neighbour(Direction::Bottom, board, i, j);
                                    let bottom: Option<&Card> = {
                                        if bottom.is_some() {
                                            board[bottom.unwrap()][j].as_ref()
                                        } else {
                                            None
                                        }
                                    };
                                    // fetch right neighbour
                                    let right =
                                        Card::get_far_neighbour(Direction::Right, board, i, j);
                                    let right: Option<&Card> = {
                                        if right.is_some() {
                                            board[i][right.unwrap()].as_ref()
                                        } else {
                                            None
                                        }
                                    };
                                    // fetch left neighbour
                                    let left =
                                        Card::get_far_neighbour(Direction::Left, board, i, j);
                                    let left: Option<&Card> = {
                                        if left.is_some() {
                                            board[i][left.unwrap()].as_ref()
                                        } else {
                                            None
                                        }
                                    };

                                    // return true if there is a neighbour which player does not own
                                    (top.is_some() && top.unwrap().player != player)
                                        || (bottom.is_some() && bottom.unwrap().player != player)
                                        || (right.is_some() && right.unwrap().player != player)
                                        || (left.is_some() && left.unwrap().player != player)
                                }
                                // other units should check out cells which have cards adjacent to them
                                _ => {
                                    let top = {
                                        if i > 0 {
                                            board[i - 1][j].as_ref()
                                        } else {
                                            None
                                        }
                                    };
                                    let bottom = {
                                        if i < 3 {
                                            board[i + 1][j].as_ref()
                                        } else {
                                            None
                                        }
                                    };
                                    let right = {
                                        if j < 4 {
                                            board[i][j + 1].as_ref()
                                        } else {
                                            None
                                        }
                                    };
                                    let left = {
                                        if j > 0 {
                                            board[i][j - 1].as_ref()
                                        } else {
                                            None
                                        }
                                    };

                                    // return true if there is an adjacent neighbour which the player does not own
                                    (top.is_some() && top.unwrap().player != player)
                                        || (bottom.is_some() && bottom.unwrap().player != player)
                                        || (right.is_some() && right.unwrap().player != player)
                                        || (left.is_some() && left.unwrap().player != player)
                                }
                            }
                        }
                    };

                    // if move has priority, add it to the beginning of the list
                    if has_priority {
                        moves.insert(0, (d, i, j));
                    }
                    // otherwise add it to the end of the list
                    else {
                        moves.push((d, i, j));
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
) -> (usize, usize, usize) {
    // init best move and scores
    let mut best_move: usize;
    let mut best_score: i8;
    let moves: Vec<(usize, usize, usize)>;

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
        let mov = moves[m];

        // make a copy of the board
        let mut temp_board: [[Option<Card>; 5]; 4] = Default::default();
        copy_board(&board, &mut temp_board);

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
        Card::place_card(board, deck1, deck2, mov.0, (mov.1, mov.2), player, bombs);

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
    let moves: Vec<(usize, usize, usize)>;
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
        let mov = moves[m];

        // make a copy of the board
        let mut temp_board: [[Option<Card>; 5]; 4] = Default::default();
        copy_board(&board, &mut temp_board);

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

        Card::place_card(board, deck1, deck2, mov.0, (mov.1, mov.2), player, bombs);

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
