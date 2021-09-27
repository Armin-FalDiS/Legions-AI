use std::io::{stdout, Write};

use crate::card::*;

macro_rules! flush {
    () => {
        stdout().flush().unwrap();
    };
}

macro_rules! input {
    ($str:expr, $msg:expr) => {
        stdin().read_line(&mut $str).expect($msg);
    };
}

pub(crate) use flush;
pub(crate) use input;

// calculates score for each player
pub fn calc_scores(board: &[[Option<Card>; 5]; 4]) -> (i8, i8) {
    let mut p1: i8 = 1;
    let mut p2: i8 = 0;

    for i in 0..4 {
        for j in 0..5 {
            let card = board[i][j].as_ref();
            match card {
                Some(c) => {
                    if c.player == 1 {
                        p1 += 1;
                    } else {
                        p2 += 1;
                    }
                }
                None => {}
            }
        }
    }

    return (p1, p2);
}

// ruturns a static evaluation of the game
pub fn evaluation(board: &[[Option<Card>; 5]; 4]) -> i8 {
    let (p1_score, p2_score) = calc_scores(&board);
    return p1_score - p2_score;
}

// copies the board from source to destination
pub fn copy_board(src: &[[Option<Card>; 5]; 4]) -> [[Option<Card>; 5]; 4] {
    let mut dst: [[Option<Card>; 5]; 4] = Default::default();
    for i in 0..4 {
        for j in 0..5 {
            if src[i][j].is_some() {
                dst[i][j] = Some(Card::copy(src[i][j].as_ref().unwrap()));
            }
        }
    }
    return dst;
}

// outputs board
pub fn show_board(board: &[[Option<Card>; 5]; 4], bombs: &[[u8; 5]; 4]) {
    for i in 0..4 {
        for j in 0..5 {
            match &board[i][j] {
                None => {
                    print!("____{}____", bombs[i][j]);
                }
                Some(card) => {
                    if card.name == Unit::Swarm {
                        let mut c = Card::copy(card);

                        let swarm = Card::swarm_count(board, card.player);
                        c.upgrade(swarm);

                        print!(
                            "{:?}({}{}{}{})[{}]",
                            c.name, c.top, c.right, c.bottom, c.left, c.player
                        );
                    } else {
                        print!(
                            "{:?}({}{}{}{})[{}]",
                            card.name, card.top, card.right, card.bottom, card.left, card.player
                        );
                    }
                }
            }
            print!("\t\t\t\t");
            flush!();
        }
        println!();
        flush!();
    }
}

// outputs player deck
pub fn show_deck(deck: &Vec<Card>, player: u8) {
    print!("Player#{} Deck:  ", player);
    let mut i: u8 = 0;
    for c in deck.iter() {
        i += 1;
        print!(
            "{}-{:?}({}{}{}{})        ",
            i, c.name, c.top, c.right, c.bottom, c.left
        );
        flush!();
    }
    println!();
}

// parses single digit entered input to (card_index, y, x)
pub fn parse_player_move(player_move: &str) -> (usize, Position) {
    let mut cm = player_move.chars();
    // determine card
    let c = cm.next();
    // determine move
    let y = cm.next();
    let x = cm.next();

    let card: usize = match c {
        Some(cc) => cc.to_digit(10).unwrap() as usize - 1,
        None => {
            panic!("Move input card index is not valid!");
        }
    };

    let position: Position = {
        let mut p: Position = (99, 99);
        match y {
            Some(yy) => {
                p.0 = yy.to_digit(10).unwrap() as usize - 1;
            }
            None => {
                panic!("Move input y is not valid!");
            }
        }

        match x {
            Some(xx) => {
                p.1 = xx.to_digit(10).unwrap() as usize - 1;
            }
            None => {
                panic!("Move input x is not valid!");
            }
        }

        p
    };

    return (card, position);
}
