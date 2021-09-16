use std::{
    cmp::{max, min, PartialEq},
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
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
    // Returns a copy of the card
    // fn copy(card: &Card) -> Card {
    //     return Card {
    //         name: card.name,
    //         top: card.top,
    //         right: card.right,
    //         bottom: card.bottom,
    //         left: card.left,
    //         player: card.player,
    //     };
    // }

    // increases every stat by n up to a maximum of 10
    fn upgrade(&mut self, n: u8) {
        self.top = min(10, self.top + n);
        self.right = min(10, self.right + n);
        self.bottom = min(10, self.bottom + n);
        self.left = min(10, self.left + n);
    }

    // decreases every stat by n down to a minimum of 1
    fn downgrade(&mut self, n: u8) {
        self.top = max(1, self.top - n);
        self.right = max(1, self.right - n);
        self.bottom = max(1, self.bottom - n);
        self.left = max(1, self.left - n);
    }

    // maps unit type to card values array
    fn get_values(unit: Unit) -> [u8; 4] {
        match unit {
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

    // placement event that needs to be run after card is placed on the board for the first time
    fn placement(
        board: &mut [[Option<Card>; 5]; 4],
        position: (usize, usize),
        bombs: &mut [[u8; 5]; 4],
    ) {
        let y = position.0;
        let x = position.1;

        match board[y][x].as_ref().unwrap().name {
            // Saboteur places bombs on empty neighbours
            Unit::Saboteur => {
                // top neighbour
                if y > 0 && board[y - 1][x].is_none() {
                    bombs[y - 1][x] += 1;
                }
                // bottom neighbour
                else if y < 3 && board[y + 1][x].is_none() {
                    bombs[y + 1][x] += 1;
                }
                // left neighbour
                else if x > 0 && board[y][x - 1].is_none() {
                    bombs[y][x - 1] += 1;
                }
                // right neighbour
                else if x < 4 && board[y][x + 1].is_none() {
                    bombs[y][x + 1] += 1;
                }
            }
            // Siren pulls cards
            Unit::Siren => {
                // top card
                for i in (0..y).rev() {
                    // if the neighbour is not free then no pulling
                    if i == y || (i == y - 1 && board[i][x].is_some()) {
                        break;
                    }
                    if board[i][x].is_some() {
                        // relocate card to neighbour cell
                        board[y - 1][x] = board[i][x].take();
                        // check for bomb on that card
                        Card::bomb_check(board, (y - 1, x), bombs);
                        // pulls only once
                        break;
                    }
                }
                // bottom card
                for i in (y + 1)..4 {
                    // if the neighbour is not free then no pulling
                    if i == y || (i == y + 1 && board[i][x].is_some()) {
                        break;
                    }
                    if board[i][x].is_some() {
                        // relocate card to neighbour cell
                        board[y + 1][x] = board[i][x].take();
                        // check for bomb on that card
                        Card::bomb_check(board, (y + 1, x), bombs);
                        // pulls only once
                        break;
                    }
                }
                // left card
                for i in (0..x).rev() {
                    // if the neighbour is not free then no pulling
                    if i == x || (i == x - 1 && board[y][i].is_some()) {
                        break;
                    }
                    if board[y][i].is_some() {
                        // relocate card to neighbour cell
                        board[y][x - 1] = board[y][i].take();
                        // check for bomb on that card
                        Card::bomb_check(board, (y, x - 1), bombs);
                        // pulls only once
                        break;
                    }
                }
                // right card
                for i in (x + 1)..5 {
                    // if the neighbour is not free then no pulling
                    if i == x || (i == x + 1 && board[y][i].is_some()) {
                        break;
                    }
                    if board[y][i].is_some() {
                        // relocate card to neighbour cell
                        board[y][x + 1] = board[y][i].take();
                        // check for bomb on that card
                        Card::bomb_check(board, (y, x + 1), bombs);
                        // pulls only once
                        break;
                    }
                }
            }
            // Titan flips adjacent cards
            Unit::Titan => {
                // top neighbour
                if y > 0 && board[y - 1][x].is_some() {
                    let card = board[y - 1][x].as_mut().unwrap();
                    let t = card.top;
                    card.top = card.bottom;
                    card.bottom = t;
                }
                // bottom neighbour
                else if y < 3 && board[y + 1][x].is_some() {
                    let card = board[y + 1][x].as_mut().unwrap();
                    let t = card.top;
                    card.top = card.bottom;
                    card.bottom = t;
                }
                // left neighbour
                else if x > 0 && board[y][x - 1].is_some() {
                    let card = board[y][x - 1].as_mut().unwrap();
                    let l = card.left;
                    card.left = card.right;
                    card.right = l;
                }
                // right neighbour
                else if x < 4 && board[y][x + 1].is_some() {
                    let card = board[y][x + 1].as_mut().unwrap();
                    let l = card.left;
                    card.left = card.right;
                    card.right = l;
                }
            }
            // Others do nothing at this stage
            _ => {}
        }

        // after card is placed, check for bombs
        Card::bomb_check(board, position, bombs);
    }

    // checks for bombs and applies effects accordingly
    fn bomb_check(
        board: &mut [[Option<Card>; 5]; 4],
        position: (usize, usize),
        bombs: &mut [[u8; 5]; 4],
    ) {
        let cell = &mut bombs[position.0][position.1];
        // check if there is a bomb
        if *cell > 0 {
            match &mut board[position.0][position.1] {
                Some(card) => {
                    // bombs detonate reducing every stat down to a minimum of 1
                    card.downgrade(*cell);
                    // all bombs are detonated
                    *cell = 0;
                }
                None => {}
            }
        }
    }

    // plays the card which is already placed on the board
    fn play(board: &mut [[Option<Card>; 5]; 4], position: (usize, usize)) {
        // if the attacker has won returns true otherwise false
        fn battle(
            attacker: Unit,
            defender: Unit,
            mut attack_value: u8,
            mut defense_value: u8,
        ) -> bool {
            // Warden has a defense bonus
            if defender == Unit::Warden {
                defense_value += 1;
            }

            // slayer uses swapped attack values
            if attacker == Unit::Slayer {
                let t = defense_value;
                defense_value = attack_value;
                attack_value = t;
            }

            // do the battle
            if attack_value > defense_value {
                // attacker wins
                return true;
            } else {
                // attacker hos lost the battle
                return false;
            }
        }

        // captured event for the defender
        fn capture_defender(defender: &mut Card, attacking_player: u8) {
            // change owner of the captured card
            defender.player = attacking_player;
            // Ravager upgrades values when captured
            if defender.name == Unit::Ravager {
                defender.upgrade(1);
            }
        }

        // capturing event for the attacker
        fn capture_attacker(attacker: &mut Card) {
            if attacker.name == Unit::Ravager {
                attacker.upgrade(1);
            }
        }

        // determine attacker position
        let y = position.0;
        let x = position.1;

        let attacking_player: u8 = { board[y][x].as_ref().unwrap().player };

        // handle the battle with top neighbour
        if y > 0 && board[y - 1][x].is_some() {
            // do the battle and save the result in capture
            let capture: bool = {
                // fetch attacker
                let a = board[y][x].as_ref().unwrap();
                // fetch defender
                let d = board[y - 1][x].as_ref().unwrap();
                // commence battle
                battle(a.name, d.name, a.top, d.bottom)
            };

            // if the attacker has won the battle then capture the card
            if capture {
                capture_defender(board[y - 1][x].as_mut().unwrap(), attacking_player);
                // trigger capture event for the attacker
                capture_attacker(board[y][x].as_mut().unwrap());
            }
        }

        // handle the battle with right neighbour
        if x < 4 && board[y][x + 1].is_some() {
            // do the battle and save the result in capture
            let capture: bool = {
                // fetch attacker
                let a = board[y][x].as_ref().unwrap();
                // fetch defender
                let d = board[y][x + 1].as_ref().unwrap();
                // commence battle
                battle(a.name, d.name, a.top, d.bottom)
            };

            // if the attacker has won the battle then capture the card
            if capture {
                capture_defender(board[y][x + 1].as_mut().unwrap(), attacking_player);
                // trigger capture event for the attacker
                capture_attacker(board[y][x].as_mut().unwrap());
            }
        }

        // handle the battle with the bottom neighbour
        if y < 3 && board[y + 1][x].is_some() {
            // do the battle and save the result in capture
            let capture: bool = {
                // fetch attacker
                let a = board[y][x].as_ref().unwrap();
                // fetch defender
                let d = board[y + 1][x].as_ref().unwrap();
                // commence battle
                battle(a.name, d.name, a.top, d.bottom)
            };

            // if the attacker has won the battle then capture the card
            if capture {
                capture_defender(board[y + 1][x].as_mut().unwrap(), attacking_player);
                // trigger capture event for the attacker
                capture_attacker(board[y][x].as_mut().unwrap());
            }
        }

        // handle the battle with the left neighbour
        if x > 0 && board[y][x - 1].is_some() {
            // do the battle and save the result in capture
            let capture: bool = {
                // fetch attacker
                let a = board[y][x].as_ref().unwrap();
                // fetch defender
                let d = board[y][x - 1].as_ref().unwrap();
                // commence battle
                battle(a.name, d.name, a.top, d.bottom)
            };

            // if the attacker has won the battle then capture the card
            if capture {
                capture_defender(board[y][x - 1].as_mut().unwrap(), attacking_player);
                // trigger capture event for the attacker
                capture_attacker(board[y][x].as_mut().unwrap());
            }
        }
    }
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

    // array holding bombs
    let mut bombs: [[u8; 5]; 4] = [[0; 5]; 4];

    // keep track of turns
    let mut turn: u8 = 0;

    // start of the game
    loop {
        // show board
        show_board(&board);

        // if there are no more cards, end the game!
        if deck1.len() + deck2.len() == 0 {
            break;
        }

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
        // player should move so apply player move on the board
        else {
            // determine player's card and move
            let player_move = parse_player_move(player_move);
            // if we can't place the card, prompt for move again
            if !place_card(
                &mut board,
                &mut deck1,
                &mut deck2,
                player_move.0,
                (player_move.1, player_move.2),
                current_turn,
                &mut bombs,
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
            print!("\t\t\t");
            flush!();
        }
        println!();
        flush!();
    }
}

// outputs player deck
fn show_deck(deck: &Vec<Card>, player: u8) {
    print!("Player#{} Deck:  ", player);
    for c in deck.iter() {
        print!(
            "{:?}({}{}{}{})        ",
            c.name, c.top, c.right, c.bottom, c.left
        );
        flush!();
    }
    println!();
}

// places a card out of the player's deck on to the board. returns success
fn place_card(
    board: &mut [[Option<Card>; 5]; 4],
    deck1: &mut Vec<Card>,
    deck2: &mut Vec<Card>,
    card: usize,
    mov: (usize, usize),
    player: u8,
    bombs: &mut [[u8; 5]; 4],
) -> bool {
    // determine the cell on the board
    let cell = &mut board[mov.0][mov.1];

    // replace cell placeholder card if move is legal
    match cell {
        // there is already a card in that cell
        Some(_c) => {
            println!("That move is not possible !");
            return false;
        }
        // cell is free to place a card
        None => {
            // player 1 plays
            if player == 1 {
                *cell = Some(deck1.remove(card));
            }
            // player 2 plays
            else {
                *cell = Some(deck2.remove(card));
            }

            Card::placement(board, mov, bombs);

            Card::play(board, mov);

            return true;
        }
    }
}

// parses single digit entered input to (card_index, y, x)
fn parse_player_move(player_move: &str) -> (usize, usize, usize) {
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

    return (card, mov / 5, mov % 5);
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
