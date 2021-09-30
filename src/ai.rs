use std::{
    cmp::{max, min},
    io::{stdout, Write},
    sync::mpsc::channel,
    thread,
};

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
    let mut best_move: usize = 0;
    let mut best_score: i8;
    let moves: Vec<(usize, usize, usize, [Option<(usize, usize)>; 4])>;

    if player == 1 {
        best_score = -120;
        moves = available_moves(board, bombs, deck1, 1);
    } else {
        best_score = 120;
        moves = available_moves(board, bombs, deck2, 2);
    }

    // determines maximum depth of minimax algorithm
    let max_depth: u8 = {
        match deck1.len() + deck2.len() {
            0..=8 => 8,
            9..=10 => 5,
            11..=12 => 4,
            _ => 3,
        }
    };

    // init channels for communication between threads
    let (tx, rx) = channel();
    
    // iterate through the moves
    for m in 0..moves.len() {
        // determine move
        let mut mov = moves[m];

        // make a copy of the board
        let mut t_board: [[Option<Card>; 5]; 4] = copy_board(&board);

        // make a clone of bombs
        let mut t_bombs = bombs.clone();

        // make a copy of the decks
        let mut t_deck1: Vec<Card> = Default::default();
        let mut t_deck2: Vec<Card> = Default::default();
        for i in 0..deck2.len() {
            t_deck2.push(Card::copy(&deck2[i]));
            if i < deck1.len() {
                t_deck1.push(Card::copy(&deck1[i]));
            }
        }

        // spawn a thread to do the calculations
        let sender = tx.clone();
        thread::spawn(move || {
            // place the card down
            Card::place_card(
                &mut t_board,
                &mut t_deck1,
                &mut t_deck2,
                mov.0,
                (mov.1, mov.2),
                player,
                &mut t_bombs,
                &mut mov.3,
            );

            let score = minimax(
                &mut t_board,
                &mut t_deck1,
                &mut t_deck2,
                &mut t_bombs,
                (player % 2) + 1,
                -125,
                125,
                max_depth,
            );

            sender
                .send((m, score))
                .expect("Thread could not send info !");
        });
    }

    // close sending channel as it is no longer needed
    drop(tx);

    print!("Progress: ");
    flush!();

    // loop through data of the recieving channel
    for data in rx {
        // break down sent data
        let (mov, score) = data;

        // flag to see if there was a better score
        let better_score: bool = {
            // maximising player
            if player == 1 {
                score > best_score
            }
            // minimising player
            else {
                score < best_score
            }
        };

        // if we have a better score, update best move
        if better_score {
            best_score = score;
            best_move = mov;
            print!("({})", best_score);
        } else {
            print!("|");
        }
        flush!();
    }

    println!();

    if (player == 1 && best_score > 100) || (player == 2 && best_score < -100) {
        println!("\n  Omae wa mou shindeiru\n");
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
            return 100 + evaluation(board);
        } else {
            return -100 + evaluation(board);
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
