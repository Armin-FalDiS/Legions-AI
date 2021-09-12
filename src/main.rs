use std::{
    io::stdin,
    io::{stdout, Write},
    panic,
};

#[derive(Clone, Copy, Debug)]
enum Unit {
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
}

impl Card {
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
    fn add_to_deck(deck: &mut Vec<Card>, card: Unit) {
        let values = Card::get_values(card);

        for i in 0..4 {
            deck.push(Card {
                name: card,
                top: values[(0 + i) % 4],
                right: values[(1 + i) % 4],
                bottom: values[(2 + i) % 4],
                left: values[(3 + i) % 4],
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
    stdout().flush().unwrap();

    // take input
    let mut deck_types = String::new();
    stdin()
        .read_line(&mut deck_types)
        .expect("You did not enter the numbers in correct format!");

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
            Card::add_to_deck(&mut deck1, unit);
        }
        // add to second player's deck
        else {
            Card::add_to_deck(&mut deck2, unit);
        }

        count += 1;
    }
}
