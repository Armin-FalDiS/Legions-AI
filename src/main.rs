use std::{
    io::{stdin, stdout, Write},
    panic,
};

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

#[derive(Clone, Copy, Debug)]
enum Unit {
    //Empty, // placeholder
    Warden,
    Siren,
    Keeper,
    Saboteur,
    Swarm,
    Slayer,
    Titan,
    Ravager,
}

struct Card {
    pub name: Unit,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
    pub player: u8,
}

impl Card {
    // maps unit type to card values array
    fn get_values(unit: Unit) -> [u8; 4] {
        match unit {
            //Unit::Empty => [0, 0, 0, 0],
            Unit::Warden => [6, 6, 4, 4],
            Unit::Siren => [7, 4, 4, 5],
            Unit::Keeper => [9, 5, 1, 5],
            Unit::Saboteur => [6, 5, 4, 5],
            Unit::Swarm => [7, 3, 3, 3],
            Unit::Slayer => [1, 6, 7, 6],
            Unit::Titan => [7, 4, 6, 3],
            Unit::Ravager => [8, 4, 6, 2],
        }
    }

    // adds a card stack (4 cards) to deck
    fn add_to_deck(deck: &mut Vec<Card>, card: Unit, player: u8) {
        let values = Card::get_values(card);

        for i in 0..4 {
            deck.push(Card {
                name: card,
                top: values[(0 + i) % 4],
                right: values[(1 + i) % 4],
                bottom: values[(2 + i) % 4],
                left: values[(3 + i) % 4],
                player,
            });
        }
    }

    fn placement() {}

    fn attack() {}

    fn endturn() {}
}

fn main() {
    // each player can only hold 8 cards
    let mut deck1: Vec<Card> = Vec::with_capacity(8);
    let mut deck2: Vec<Card> = Vec::with_capacity(8);

    // prompt to input deck types
    print!(
        "
    1\tWarden
    2\tSiren
    3\tKeeper
    4\tSaboteur
    5\tSwarm
    6\tSlayer
    7\tTitan
    8\tRavager\n
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
            "2" => Unit::Siren,
            "3" => Unit::Keeper,
            "4" => Unit::Saboteur,
            "5" => Unit::Swarm,
            "6" => Unit::Slayer,
            "7" => Unit::Titan,
            "8" => Unit::Ravager,
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

    // keep track of turns
    let mut turn: u8 = 0;

    // start of the game
    loop {
        // show board
        show_board(&board);

        // show player1 deck
        show_deck(&deck1, 1);
        // show player2 deck
        show_deck(&deck2, 2);

        // determine current turn's player
        let current_turn = (turn % 2) + 1;

        // prompt move input
        print!("It's Player#{}'s turn. Choose Card & Place: ", current_turn);
        flush!();

        // take move input
        let mut player_move = String::new();
        input!(player_move, "Invalid move input!");
        let player_move = player_move.trim();

        // ai should play
        if player_move == "ai" || player_move == "0" {
            ai(&mut board, &mut deck1, &mut deck2, current_turn);
        }
        // apply player move on the board
        else {
            // if we can't place card, prompt for move again
            if !place_card(
                &mut board,
                &mut deck1,
                &mut deck2,
                player_move,
                current_turn,
            ) {
                continue;
            }
        }

        // next turn
        turn += 1;
    }
}

// outputs board
fn show_board(board: &[[Option<Card>; 5]; 4]) {
    for i in 0..4 {
        for j in 0..5 {
            match &board[i][j] {
                None => {
                    print!("_________");
                }
                Some(card) => {
                    print!("{:?}({})", card.name, card.player);
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
fn show_deck(deck: &Vec<Card>, player: u8) {
    print!("Player#{} Deck: ", player);
    for c in deck.iter() {
        print!("{:?}({}{}{}{})\t\t", c.name, c.top, c.right, c.bottom, c.left);
        flush!();
    }
    println!();
}

// places a card out of the player's deck on to the board. returns success
fn place_card(
    board: &mut [[Option<Card>; 5]; 4],
    deck1: &mut Vec<Card>,
    deck2: &mut Vec<Card>,
    player_move: &str,
    player: u8,
) -> bool {
    let mut cm = player_move.split(' ');
    // determine card
    let c = cm.next();
    // determine move
    let m = cm.next();

    let card: usize;
    let mov: usize;

    match c {
        Some(cc) => {
            card = cc.parse::<usize>().unwrap() - 1;
        }
        None => {
            panic!("Move input is not valid!");
        }
    };

    match m {
        Some(mm) => {
            mov = mm.parse::<usize>().unwrap() - 1;
        }
        None => {
            panic!("Move input is not valid!");
        }
    }

    // determine the cell on the board
    let cell = &mut board[mov / 4][mov % 5];

    // replace cell placeholder card if move is legal
    match cell {
        Some(_c) => {
            println!("That move is not possible !");
            return false;
        }
        None => {
            // player 1 plays
            if player == 1 {
                *cell = Some(deck1.remove(card));
            }
            // player 2 plays
            else {
                *cell = Some(deck2.remove(card));
            }
            return true;
        }
    }
}

// plays the best move on the board for current player
fn ai(
    board: &mut [[Option<Card>; 5]; 4],
    deck1: &mut Vec<Card>,
    deck2: &mut Vec<Card>,
    player: u8,
) {
    return;
}
