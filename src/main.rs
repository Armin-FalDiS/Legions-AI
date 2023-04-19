use std::{
    io::{stdin, stdout, Write},
    panic,
};

mod ai;
mod card;
mod utility;

use ai::ai;
use card::*;
use utility::*;

fn main() {
    // each player can only hold 8 cards
    let mut deck1: Vec<Card> = Vec::with_capacity(8);
    let mut deck2: Vec<Card> = Vec::with_capacity(8);

    // prompt to input deck types
    print!(
        "
    1\tWarden
    2\tKeeper
    3\tSiren
    4\tSaboteur
    5\tRavager
    6\tTitan
    7\tSlayer
    8\tSwarm
    9\tLancer\n
    Enter players deck unit types (player1 player1 player2 player2): "
    );
    flush!();

    // take input
    let mut deck_types = String::new();
    input!(
        deck_types,
        "You did not enter the numbers in correct format!"
    );

    // count which deck to insert
    let mut count: u8 = 0;
    // create decks for each player based on input
    for d in deck_types.trim().split(' ') {
        // map input to determine card unit type
        let unit: Unit = match d {
            "1" => Unit::Warden,
            "2" => Unit::Keeper,
            "3" => Unit::Siren,
            "4" => Unit::Saboteur,
            "5" => Unit::Ravager,
            "6" => Unit::Titan,
            "7" => Unit::Slayer,
            "8" => Unit::Swarm,
            "9" => Unit::Lancer,
            _ => panic!("Invalid deck digit!"),
        };

        // add to first player's deck
        if count < 2 {
            Card::add_to_deck(&mut deck1, unit, 1);
        }
        // add to second player's deck
        else {
            Card::add_to_deck(&mut deck2, unit, 2);
        }

        count += 1;
    }

    // init Empty board (height 4, width 5)
    let mut board: [[Option<Card>; 5]; 4] = Default::default();

    // array holding bombs
    let mut bombs: [[u8; 5]; 4] = [[0; 5]; 4];

    // keep track of turns
    let mut turn: u8 = 0;

    // keep track of last move (board, bombs, card_ind, card)
    let mut prev_state: Option<([[Option<Card>; 5]; 4], [[u8; 5]; 4], usize, Card)> =
        Default::default();

    println!("** Board is shown as (y,x) and the number of bombs that the cell holds **");

    // start of the game
    loop {
        println!();
        // show board
        show_board(&board, &bombs);

        // if there are no more cards, end the game!
        if deck1.len() + deck2.len() == 0 {
            // show scores and the winner
            let scoreboard = calc_scores(&board);
            println!(
                "\n
            ************************************
            *       The score is {} to {}
            *       Player {} WINS !
            ************************************
            \n",
                scoreboard.0,
                scoreboard.1,
                {
                    if scoreboard.0 > scoreboard.1 {
                        1
                    } else {
                        2
                    }
                }
            );
            // end game
            break;
        }

        println!();
        // show player1 deck
        show_deck(&deck1, 1);
        // show player2 deck
        show_deck(&deck2, 2);
        println!();

        // determine current turn's player
        let current_turn = (turn % 2) + 1;

        // prompt move input
        print!(
            "It's Player#{}'s turn. Choose Card & Place (CardIndexYX e.g. 111): ",
            current_turn
        );
        flush!();

        // take move input
        let mut player_move = String::new();
        input!(player_move, "Invalid move input!");
        let player_move = player_move.trim();

        // undo move
        if player_move == "b" && turn > 0 {
            if prev_state.is_some() {
                println!("\nUndoing move ...\n");
                let state = prev_state.take().unwrap();
                board = state.0;
                bombs = state.1;
                // previous turn was player1's
                if current_turn == 2 {
                    deck1.insert(state.2, state.3);
                }
                // previous turn was player2's
                else {
                    deck2.insert(state.2, state.3);
                }

                turn -= 1;
                continue;
            }
        }
        // ai should play
        else if player_move.is_empty() {
            let mut ai_move = ai(&mut board, &mut deck1, &mut deck2, current_turn, &mut bombs);

            // fetch a copy of the played card
            let prev_card = {
                if current_turn == 1 {
                    Card::copy(&deck1[ai_move.0])
                } else {
                    Card::copy(&deck2[ai_move.0])
                }
            };

            // announce AI move
            println!(
                "\nAI placed card a {:?}({}{}{}{}) on {}, {}\n",
                prev_card.name,
                prev_card.top,
                prev_card.right,
                prev_card.bottom,
                prev_card.left,
                ai_move.1 + 1,
                ai_move.2 + 1
            );

            // save prev state
            prev_state = Some((copy_board(&board), bombs.clone(), ai_move.0, prev_card));

            Card::place_card(
                &mut board,
                &mut deck1,
                &mut deck2,
                ai_move.0,
                (ai_move.1, ai_move.2),
                current_turn,
                &mut bombs,
                &mut ai_move.3,
            );
        }
        // player should move so apply player move on the board
        else {
            // determine player's card and move
            let player_move = parse_player_move(player_move);

            // save prev state
            let prev_card = {
                if current_turn == 1 {
                    Card::copy(&deck1[player_move.0])
                } else {
                    Card::copy(&deck2[player_move.0])
                }
            };
            let card = prev_card.name;
            prev_state = Some((copy_board(&board), bombs.clone(), player_move.0, prev_card));

            // fetch neighbours of this move
            let mut neighbours =
                Card::get_neighbours(&board, player_move.1 .0, player_move.1 .1, card);

            // if we can't place the card, prompt for move again
            if !Card::place_card(
                &mut board,
                &mut deck1,
                &mut deck2,
                player_move.0,
                player_move.1,
                current_turn,
                &mut bombs,
                &mut neighbours,
            ) {
                continue;
            }
        }

        // next turn
        turn += 1;
    }
}
