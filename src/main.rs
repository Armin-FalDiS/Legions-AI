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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
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
    fn copy(card: &Card) -> Card {
        return Card {
            name: card.name,
            top: card.top,
            right: card.right,
            bottom: card.bottom,
            left: card.left,
            player: card.player,
        };
    }

    // increases every stat by n up to a maximum of 10
    fn upgrade(&mut self, n: u8) {
        self.top = min(10, self.top + n);
        self.right = min(10, self.right + n);
        self.bottom = min(10, self.bottom + n);
        self.left = min(10, self.left + n);
    }

    // decreases every stat by n down to a minimum of 1
    fn downgrade(&mut self, n: u8) {
        self.top = max(1, self.top as i8 - n as i8) as u8;
        self.right = max(1, self.right as i8 - n as i8) as u8;
        self.bottom = max(1, self.bottom as i8 - n as i8) as u8;
        self.left = max(1, self.left as i8 - n as i8) as u8;
    }

    // maps unit type to card values array
    fn get_values(unit: Unit) -> [[u8; 4]; 4] {
        match unit {
            Unit::Warden => [[6, 6, 4, 4], [4, 6, 6, 4], [4, 4, 6, 6], [6, 4, 4, 6]],
            Unit::Siren => [[7, 5, 4, 4], [4, 7, 5, 4], [4, 4, 7, 5], [5, 4, 4, 7]],
            Unit::Keeper => [[9, 5, 1, 5], [5, 9, 5, 1], [1, 5, 9, 5], [5, 1, 5, 9]],
            Unit::Saboteur => [[6, 5, 4, 5], [5, 6, 5, 4], [4, 5, 6, 5], [5, 4, 5, 6]],
            Unit::Swarm => [[7, 3, 3, 3], [3, 7, 3, 3], [3, 3, 7, 3], [3, 3, 3, 7]],
            Unit::Slayer => [[1, 6, 7, 6], [6, 1, 6, 7], [7, 6, 1, 6], [6, 7, 6, 1]],
            Unit::Titan => [[7, 3, 6, 4], [4, 7, 3, 6], [6, 4, 7, 3], [3, 6, 4, 7]],
            Unit::Ravager => [[8, 2, 6, 4], [4, 8, 2, 6], [6, 4, 8, 2], [2, 6, 4, 8]]
        }
    }

    // adds a card stack (4 cards) to deck
    fn add_to_deck(deck: &mut Vec<Card>, card: Unit, player: u8) {
        let values = Card::get_values(card);

        for i in values {
            deck.push(Card {
                name: card,
                top: i[0],
                right: i[1],
                bottom: i[2],
                left: i[3],
                player,
            });
        }
    }

    // gets the first neighbour in specified direction
    fn get_far_neighbour(
        direction: Direction,
        board: &[[Option<Card>; 5]; 4],
        y: usize,
        x: usize,
    ) -> Option<usize> {
        let iter: Box<dyn Iterator<Item = usize>> = {
            match direction {
                Direction::Top => Box::new((0..y).rev()),
                Direction::Right => Box::new((x + 1)..5) as Box<dyn Iterator<Item = usize>>,
                Direction::Bottom => Box::new((y + 1)..4) as Box<dyn Iterator<Item = usize>>,
                Direction::Left => Box::new((0..x).rev()),
            }
        };

        // iterate through that direction
        for i in iter {
            // determine y & x based on direction
            let ny = if direction == Direction::Top || direction == Direction::Bottom {
                i
            } else {
                y
            };
            let nx = if direction == Direction::Right || direction == Direction::Left {
                i
            } else {
                x
            };
            // if we found a card
            if board[ny][nx].is_some() {
                if direction == Direction::Top || direction == Direction::Bottom {
                    return Some(ny); // top or bottom returns y (x is same as main card)
                } else {
                    return Some(nx); // left or right returns x (y is same as main card)
                }
            }
        }

        return None;
    }

    // pulls a card to the neighbouring cell
    fn pull(
        y: usize,
        x: usize,
        direction: Direction,
        board: &mut [[Option<Card>; 5]; 4],
        bombs: &mut [[u8; 5]; 4],
    ) {
        // fetch neighbour
        let neighbour = Card::get_far_neighbour(direction, board, y, x);

        // variable to track bomb damage when pulled
        let mut damage: u8 = 0;

        // check if neighbour is adjecent
        let is_adjacent: bool = {
            if neighbour.is_none() {
                false
            } else {
                let neighbour = *neighbour.as_ref().unwrap();
                (direction == Direction::Top && neighbour == y - 1)
                    || (direction == Direction::Right && neighbour == x + 1)
                    || (direction == Direction::Bottom && neighbour == y + 1)
                    || (direction == Direction::Left && neighbour == x - 1)
            }
        };

        // if there exist a non adjacent neighbour
        if neighbour.is_some() && !is_adjacent {
            let neighbour = *neighbour.as_ref().unwrap();
            let iter = {
                match direction {
                    Direction::Top => (neighbour + 1)..y,
                    Direction::Right => (x + 1)..neighbour,
                    Direction::Bottom => (y + 1)..neighbour,
                    Direction::Left => (neighbour + 1)..x,
                }
            };

            // flag showing whether the direction is vertical
            let dir_vertical: bool = {
                match direction {
                    Direction::Top | Direction::Bottom => true,
                    Direction::Left | Direction::Right => false,
                }
            };

            // handle bombs
            for i in iter {
                if dir_vertical {
                    // increase damage
                    damage += bombs[i][x];
                    // detonate all bombs in path cell
                    bombs[i][x] = 0;
                } else {
                    damage += bombs[y][i];
                    bombs[y][i] = 0;
                }
            }

            // apply damage and relocate card
            if dir_vertical {
                // apply damage to the card
                board[neighbour][x].as_mut().unwrap().downgrade(damage);

                // relocate the card to the neighbouring cell
                if direction == Direction::Top {
                    board[y - 1][x] = board[neighbour][x].take();
                } else {
                    board[y + 1][x] = board[neighbour][x].take();
                }
            } else {
                board[y][neighbour].as_mut().unwrap().downgrade(damage);

                if direction == Direction::Right {
                    board[y][x + 1] = board[y][neighbour].take();
                } else {
                    board[y][x - 1] = board[y][neighbour].take();
                }
            }
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
                if y < 3 && board[y + 1][x].is_none() {
                    bombs[y + 1][x] += 1;
                }
                // left neighbour
                if x > 0 && board[y][x - 1].is_none() {
                    bombs[y][x - 1] += 1;
                }
                // right neighbour
                if x < 4 && board[y][x + 1].is_none() {
                    bombs[y][x + 1] += 1;
                }
            }
            // Siren pulls cards
            Unit::Siren => {
                // pull top card
                Card::pull(y, x, Direction::Top, board, bombs);
                // pull right card
                Card::pull(y, x, Direction::Right, board, bombs);
                // pull bottom card
                Card::pull(y, x, Direction::Bottom, board, bombs);
                // pull left card
                Card::pull(y, x, Direction::Left, board, bombs);
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

    // checks for bombs and applies damage accordingly
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

    // returns the count of player owned swarms
    fn swarm_count(board: &[[Option<Card>; 5]; 4], player: u8) -> u8 {
        let mut swarm: u8 = 0;
        for i in 0..4 {
            for j in 0..5 {
                if board[i][j].is_some() {
                    let card = board[i][j].as_ref().unwrap();
                    if card.name == Unit::Swarm && card.player == player {
                        swarm += 1;
                    }
                }
            }
        }
        // swarm gets a bonus for each ally swarm except self
        if swarm > 0 {
            return swarm - 1;
        } else {
            return swarm;
        }
    }

    // play event that need to be run when the card is already placed on the board
    fn play(board: &mut [[Option<Card>; 5]; 4], y: usize, x: usize, same_chain: bool) {
        // println!(
        //     " ~~~ Play with {:?} @ {}:{} with same = {}",
        //     board[position.0][position.1].as_ref().unwrap().name,
        //     position.0,
        //     position.1,
        //     same
        // );

        fn battle(
            board: &mut [[Option<Card>; 5]; 4],
            y: usize,
            x: usize,
            direction: Direction,
            same: bool,
            same_chain: bool,
        ) {
            // determine the attacker
            let attacking_player: u8 = { board[y][x].as_ref().unwrap().player };

            // fetch neighbour
            let neighbour_position: Option<(usize, usize)> = {
                let card = { board[y][x].as_ref().unwrap().name };
                match direction {
                    Direction::Top => {
                        if y > 0 {
                            if card == Unit::Keeper {
                                let n = Card::get_far_neighbour(direction, board, y, x);
                                if n.is_none() {
                                    None
                                } else {
                                    Some((*n.as_ref().unwrap(), x))
                                }
                            } else {
                                Some((y - 1, x))
                            }
                        } else {
                            None
                        }
                    }
                    Direction::Right => {
                        if x < 4 {
                            if card == Unit::Keeper {
                                let n = Card::get_far_neighbour(direction, board, y, x);
                                if n.is_none() {
                                    None
                                } else {
                                    Some((y, *n.as_ref().unwrap()))
                                }
                            } else {
                                Some((y, x + 1))
                            }
                        } else {
                            None
                        }
                    }
                    Direction::Bottom => {
                        if y < 3 {
                            if card == Unit::Keeper {
                                let n = Card::get_far_neighbour(direction, board, y, x);
                                if n.is_none() {
                                    None
                                } else {
                                    Some((*n.as_ref().unwrap(), x))
                                }
                            } else {
                                Some((y + 1, x))
                            }
                        } else {
                            None
                        }
                    }
                    Direction::Left => {
                        if x > 0 {
                            if card == Unit::Keeper {
                                let n = Card::get_far_neighbour(direction, board, y, x);
                                if n.is_none() {
                                    None
                                } else {
                                    Some((y, *n.as_ref().unwrap()))
                                }
                            } else {
                                Some((y, x - 1))
                            }
                        } else {
                            None
                        }
                    }
                }
            };
            let neighbour = {
                if neighbour_position.is_some() {
                    let n = *neighbour_position.as_ref().unwrap();
                    board[n.0][n.1].as_ref()
                } else {
                    None
                }
            };

            if neighbour.is_some() {
                // do the battle and save the result in capture
                let capture: bool = {
                    // fetch attacker
                    let a = board[y][x].as_ref().unwrap();
                    // fetch defender
                    let d = neighbour.unwrap();
                    // if attacking player owns this card, there is no battle
                    if d.player == attacking_player {
                        false
                    } else {
                        // if same effect is present, no battle is needed
                        if same {
                            true
                        }
                        // otherwise let them fight normally as usual
                        else {
                            match direction {
                                Direction::Top => {
                                    fight(a.name, d.name, a.top, d.bottom, board, attacking_player)
                                }
                                Direction::Right => {
                                    fight(a.name, d.name, a.right, d.left, board, attacking_player)
                                }
                                Direction::Bottom => {
                                    fight(a.name, d.name, a.bottom, d.top, board, attacking_player)
                                }
                                Direction::Left => {
                                    fight(a.name, d.name, a.left, d.right, board, attacking_player)
                                }
                            }
                        }
                    }
                };

                // if the attacker has won the battle then capture the card
                if capture {
                    let n = *neighbour_position.as_ref().unwrap();
                    capture_defender(board[n.0][n.1].as_mut().unwrap(), attacking_player);
                    // trigger capture event for the attacker
                    capture_attacker(board[y][x].as_mut().unwrap());
                    if same || same_chain {
                        Card::play(board, n.0, n.1, true);
                    }
                }
            }
        }

        // if the attacker has won returns true otherwise false
        fn fight(
            attacker: Unit,
            defender: Unit,
            mut attack_value: u8,
            mut defense_value: u8,
            board: &[[Option<Card>; 5]; 4],
            attacking_player: u8,
        ) -> bool {
            match defender {
                // Warden has a defense bonus
                Unit::Warden => {
                    defense_value += 1;
                }
                // Swarm gets a bonus
                Unit::Swarm => {
                    let defending_player = (attacking_player % 2) + 1;
                    defense_value = min(
                        10,
                        defense_value + Card::swarm_count(board, defending_player),
                    );
                }
                _ => {}
            }

            match attacker {
                // Slayer uses swapped attack values
                Unit::Slayer => {
                    let t = defense_value;
                    defense_value = attack_value;
                    attack_value = t;
                }
                // Swarm gets ally bonus
                Unit::Swarm => {
                    attack_value = min(
                        10,
                        attack_value + Card::swarm_count(board, attacking_player),
                    );
                }
                _ => {}
            }

            // println!(
            //     " ~~~ Battle between {:?} & {:?} with {} vs {} repectively",
            //     attacker, defender, attack_value, defense_value
            // );

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

        // check for same rule
        let mut top_same: bool = false;
        let mut right_same: bool = false;
        let mut bottom_same: bool = false;
        let mut left_same: bool = false;

        // if we are not in a same chain effect, check for sames
        if !same_chain {
            // extract neighbouring values
            let top = {
                if y == 0 {
                    0
                } else {
                    let t = board[y - 1][x].as_ref();
                    if t.is_some() {
                        t.unwrap().bottom
                    } else {
                        0
                    }
                }
            };
            let right = {
                if x == 4 {
                    0
                } else {
                    let r = board[y][x + 1].as_ref();
                    if r.is_some() {
                        r.unwrap().left
                    } else {
                        0
                    }
                }
            };
            let bottom = {
                if y == 3 {
                    0
                } else {
                    let b = board[y + 1][x].as_ref();
                    if b.is_some() {
                        b.unwrap().top
                    } else {
                        0
                    }
                }
            };
            let left = {
                if x == 0 {
                    0
                } else {
                    let l = board[y][x - 1].as_ref();
                    if l.is_some() {
                        l.unwrap().right
                    } else {
                        0
                    }
                }
            };

            // get the attacking card
            let me = board[y][x].as_ref().unwrap();

            // keep same count
            let mut same_count: u8 = 0;

            // check for same and raise flags accordingly
            if top != 0 && top == me.top {
                top_same = true;
                same_count += 1;
            }
            if right != 0 && right == me.right {
                right_same = true;
                same_count += 1;
            }
            if bottom != 0 && bottom == me.bottom {
                bottom_same = true;
                same_count += 1;
            }
            if left != 0 && left == me.left {
                left_same = true;
                same_count += 1;
            }

            // println!(
            //     " ~~~ There were {} sames as such: t= {}, r= {}, b= {}, l= {}",
            //     same_count, top_same, right_same, bottom_same, left_same
            // );

            // if there are less than 2 sames, then same effect is canceled
            if same_count < 2 {
                top_same = false;
                right_same = false;
                bottom_same = false;
                left_same = false;
            }
        }

        // handle the battle with top neighbour
        battle(board, y, x, Direction::Top, top_same, same_chain);
        // handle the battle with right neighbour
        battle(board, y, x, Direction::Right, right_same, same_chain);
        // handle the battle with the bottom neighbour
        battle(board, y, x, Direction::Bottom, bottom_same, same_chain);
        // handle the battle with the left neighbour
        battle(board, y, x, Direction::Left, left_same, same_chain);
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
        print!("It's Player#{}'s turn. Choose Card & Place: ", current_turn);
        flush!();

        // take move input
        let mut player_move = String::new();
        input!(player_move, "Invalid move input!");
        let player_move = player_move.trim();

        // ai should play
        if player_move.is_empty() {
            let ai_move = ai(&mut board, &mut deck1, &mut deck2, current_turn, &mut bombs);

            place_card(
                &mut board,
                &mut deck1,
                &mut deck2,
                ai_move.0,
                (ai_move.1, ai_move.2),
                current_turn,
                &mut bombs,
            );

            println!("\nAI moved on {}, {}\n", ai_move.1, ai_move.2);
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
fn show_board(board: &[[Option<Card>; 5]; 4], bombs: &[[u8; 5]; 4]) {
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
fn show_deck(deck: &Vec<Card>, player: u8) {
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
        Some(c) => {
            println!(
                "{}-{} is already occupied with a {:?} !",
                mov.0, mov.1, c.name
            );
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

            Card::play(board, mov.0, mov.1, false);

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

// calculates score for each player
fn calc_scores(board: &[[Option<Card>; 5]; 4]) -> (i8, i8) {
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

// copies the board from source to destination
fn copy_board(src: &[[Option<Card>; 5]; 4], dst: &mut [[Option<Card>; 5]; 4]) {
    for i in 0..4 {
        for j in 0..5 {
            if src[i][j].is_some() {
                dst[i][j] = Some(Card::copy(src[i][j].as_ref().unwrap()));
            }
        }
    }
}

// returns a vector of (card, row, column) of available moves
fn available_moves(board: &[[Option<Card>; 5]; 4], deck: &Vec<Card>, player: u8) -> Vec<(usize, usize, usize)> {
    let mut moves: Vec<(usize, usize, usize)> = Default::default();

    // iterate through the boards for each card in deck
    for i in 0..4 {
        for j in 0..5 {
            for d in 0..deck.len() {
                if board[i][j].is_none() {
                    // flag to determine whether the move has priority
                    let has_priority: bool = {
                        match deck[d].name {
                            // Keeper and Siren affect cards at range so having a far neighbour is could be good
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
                                let bottom = Card::get_far_neighbour(Direction::Bottom, board, i, j);
                                let bottom: Option<&Card> = {
                                    if bottom.is_some() {
                                        board[bottom.unwrap()][j].as_ref()
                                    } else {
                                        None
                                    }
                                };
                                // fetch right neighbour
                                let right = Card::get_far_neighbour(Direction::Right, board, i, j);
                                let right: Option<&Card> = {
                                    if right.is_some() {
                                        board[i][right.unwrap()].as_ref()
                                    } else {
                                        None
                                    }
                                };
                                // fetch left neighbour
                                let left = Card::get_far_neighbour(Direction::Left, board, i, j);
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
fn ai(
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
        best_score = -100;
        best_move = 0;
        moves = available_moves(board, deck1, 1);
    } else {
        best_score = 100;
        best_move = 0;
        moves = available_moves(board, deck2, 2);
    }

    // determines maximum depth of minimax algorithm
    let max_depth: u8 = {
        match deck1.len() + deck2.len() {
            0..=7 => 5,
            8..=11 => 4,
            _ => 3,
        }
    };

    // iterate through the moves
    for m in 0..moves.len() {
        println!(" === Progress: {}/{}", m, moves.len());

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
        place_card(board, deck1, deck2, mov.0, (mov.1, mov.2), player, bombs);

        // maximising player
        if player == 1 {
            // calculate score for this move
            let score = minimax(board, deck1, deck2, bombs, 2, -100, 100, max_depth);

            if score > best_score {
                println!(
                    " === New HIGH score for MAX @ {}, {} !! It's {}",
                    mov.1, mov.2, score
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
            let score = minimax(board, deck1, deck2, bombs, 1, -100, 100, max_depth);

            if score < best_score {
                println!(
                    " === New HIGH score for MIN @ {}, {} !! It's {}",
                    mov.1, mov.2, score
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
    let player_scores = calc_scores(&board);

    // check if the game is over
    if deck1.is_empty() && deck2.is_empty() {
        // player 1 wins
        if player_scores.0 > player_scores.1 {
            // println!(" ### Player1's win @ depth {}", depth);
            return 120;
        }
        // player 2 wins
        else {
            // println!(" ### Player2's win @ depth {}", depth);
            return -120;
        }
    }

    // if we are out of depth, return static evaluation
    if depth == 0 {
        return player_scores.0 - player_scores.1;
    }

    // get all possible moves & init score
    let moves: Vec<(usize, usize, usize)>;
    let mut best_score: i8;
    if player == 1 {
        moves = available_moves(board, deck1, 1);
        best_score = -100;
    } else {
        moves = available_moves(board, deck2, 2);
        best_score = 100;
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

        place_card(board, deck1, deck2, mov.0, (mov.1, mov.2), player, bombs);

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
