use std::{
    cmp::{max, min},
    mem::swap,
};

pub type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Unit {
    Warden,
    Keeper,
    Siren,
    Saboteur,
    Ravager,
    Titan,
    Slayer,
    Swarm,
    Lancer,
}

pub struct Card {
    pub name: Unit,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
    pub player: u8,
}

impl Card {
    // Returns a copy of the card
    pub fn copy(card: &Card) -> Card {
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
    pub fn upgrade(&mut self, n: u8) {
        self.top = min(10, self.top + n);
        self.right = min(10, self.right + n);
        self.bottom = min(10, self.bottom + n);
        self.left = min(10, self.left + n);
    }

    // decreases every stat by n down to a minimum of 1
    pub fn downgrade(&mut self, n: u8) {
        self.top = max(1, self.top as i8 - n as i8) as u8;
        self.right = max(1, self.right as i8 - n as i8) as u8;
        self.bottom = max(1, self.bottom as i8 - n as i8) as u8;
        self.left = max(1, self.left as i8 - n as i8) as u8;
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
            Unit::Lancer => [6, 4, 6, 4],
        }
    }

    // adds a card stack (4 cards) to deck
    pub fn add_to_deck(deck: &mut Vec<Card>, card: Unit, player: u8) {
        let values = Card::get_values(card);

        for i in 0..4 {
            if player == 1 {
                deck.push(Card {
                    name: card,
                    top: values[(0 + (4 - i)) % 4],
                    right: values[(1 + (4 - i)) % 4],
                    bottom: values[(2 + (4 - i)) % 4],
                    left: values[(3 + (4 - i)) % 4],
                    player,
                });
            } else {
                // player 2 uses mirrored cards
                let mut card = Card {
                    name: card,
                    top: values[(0 + i) % 4],
                    right: values[(1 + i) % 4],
                    bottom: values[(2 + i) % 4],
                    left: values[(3 + i) % 4],
                    player,
                };

                swap(&mut card.left, &mut card.right);

                deck.push(card);
            }
        }
    }

    // gets the first neighbour in specified direction
    pub fn get_far_neighbour(
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
    pub fn pull(
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

    // places a card out of the player's deck on to the board. returns success
    pub fn place_card(
        board: &mut [[Option<Card>; 5]; 4],
        deck1: &mut Vec<Card>,
        deck2: &mut Vec<Card>,
        card: usize,
        mov: Position,
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

    // placement event that needs to be run after card is placed on the board for the first time
    pub fn placement(
        board: &mut [[Option<Card>; 5]; 4],
        position: Position,
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
                    swap(&mut card.top, &mut card.bottom);
                }
                // bottom neighbour
                if y < 3 && board[y + 1][x].is_some() {
                    let card = board[y + 1][x].as_mut().unwrap();
                    swap(&mut card.bottom, &mut card.top);
                }
                // left neighbour
                if x > 0 && board[y][x - 1].is_some() {
                    let card = board[y][x - 1].as_mut().unwrap();
                    swap(&mut card.left, &mut card.right);
                }
                // right neighbour
                if x < 4 && board[y][x + 1].is_some() {
                    let card = board[y][x + 1].as_mut().unwrap();
                    swap(&mut card.right, &mut card.left);
                }
            }
            // Others do nothing at this stage
            _ => {}
        }

        // after card is placed, check for bombs
        Card::bomb_check(board, position, bombs);
    }

    // checks for bombs and applies damage accordingly
    pub fn bomb_check(
        board: &mut [[Option<Card>; 5]; 4],
        position: Position,
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
    pub fn swarm_count(board: &[[Option<Card>; 5]; 4], player: u8) -> u8 {
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
    pub fn play(board: &mut [[Option<Card>; 5]; 4], y: usize, x: usize, combo: bool) {
        // result of the fight between two cards
        #[derive(Clone, Copy, PartialEq, Debug)]
        enum FightResult {
            Win,
            Tie,
            Lose,
        }

        // handles battle with neighbour in specified direction and returns the result
        fn battle(
            board: &mut [[Option<Card>; 5]; 4],
            y: usize,
            x: usize,
            direction: Direction,
        ) -> (Option<FightResult>, Position) {
            // determine the attacker
            let attacking_player: u8 = { board[y][x].as_ref().unwrap().player };

            // fetch neighbour
            let neighbour_position: Option<Position> = {
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

            // if neighbour exists, check it
            if neighbour.is_some() {
                // fetch attacker
                let a = board[y][x].as_ref().unwrap();
                // fetch defender
                let d = neighbour.unwrap();

                let n = neighbour_position.unwrap();
                match direction {
                    Direction::Top => {
                        return (
                            Some(fight(
                                a.name,
                                d.name,
                                a.top,
                                d.bottom,
                                board,
                                attacking_player,
                                d.player,
                            )),
                            (n.0, n.1),
                        );
                    }
                    Direction::Right => {
                        return (
                            Some(fight(
                                a.name,
                                d.name,
                                a.right,
                                d.left,
                                board,
                                attacking_player,
                                d.player,
                            )),
                            (n.0, n.1),
                        );
                    }
                    Direction::Bottom => {
                        return (
                            Some(fight(
                                a.name,
                                d.name,
                                a.bottom,
                                d.top,
                                board,
                                attacking_player,
                                d.player,
                            )),
                            (n.0, n.1),
                        );
                    }
                    Direction::Left => {
                        return (
                            Some(fight(
                                a.name,
                                d.name,
                                a.left,
                                d.right,
                                board,
                                attacking_player,
                                d.player,
                            )),
                            (n.0, n.1),
                        );
                    }
                }
            }

            // if neighbour is empty or no fight was fought, return None
            return (None, (99, 99));
        }

        // fight the opponent card and returns result
        fn fight(
            attacker: Unit,
            defender: Unit,
            mut attack_value: u8,
            mut defense_value: u8,
            board: &[[Option<Card>; 5]; 4],
            attacking_player: u8,
            defending_player: u8,
        ) -> FightResult {
            match defender {
                // Warden has a defense bonus ONLY against enemies
                Unit::Warden => {
                    if attacking_player != defending_player {
                        defense_value += 1;
                    }
                }
                // Swarm gets ally bonus
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
                    swap(&mut defense_value, &mut attack_value);
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

            // do the battle
            if attack_value > defense_value {
                // attacker wins
                return FightResult::Win;
            } else if attack_value == defense_value {
                // it's a tie
                return FightResult::Tie;
            } else {
                // attacker hos lost the battle
                return FightResult::Lose;
            }
        }

        // handles the result of the battle triggering capture event accordingly
        fn handle_result(
            result: FightResult,
            same: u8,
            position: Position,
            neighbour_position: Position,
            board: &mut [[Option<Card>; 5]; 4],
            combo: bool,
            direction: Direction,
            pierce: bool,
        ) {
            // println!(
            //     "Handling a {:?} @ {}, {} vs {}, {} towards {:?} with same_count = {}, combo = {}, pierce = {}",
            //     result,
            //     position.0 + 1,
            //     position.1 + 1,
            //     neighbour_position.0 + 1,
            //     neighbour_position.1 + 1,
            //     direction,
            //     same,
            //     combo,
            //     pierce
            // );
            match result {
                FightResult::Win => {
                    // capture neighbour when the battle is won
                    capture_event(neighbour_position, position, board, combo);
                    // Lancer has pierce ability
                    if !pierce
                        && board[position.0][position.1].as_ref().unwrap().name == Unit::Lancer
                    {
                        // pierce attacks neighbour's neighbour
                        // we swap places with neighbour to fight the desired card
                        let temp_neighbour =
                            board[neighbour_position.0][neighbour_position.1].take();
                        board[neighbour_position.0][neighbour_position.1] =
                            board[position.0][position.1].take();
                        // commence battle at the neighbour's position
                        let battle_result =
                            battle(board, neighbour_position.0, neighbour_position.1, direction);
                        // check if there was a battle
                        if battle_result.0.is_some() {
                            // handle the result
                            handle_result(
                                battle_result.0.unwrap(),
                                0,
                                neighbour_position,
                                battle_result.1,
                                board,
                                combo,
                                direction,
                                true,
                            );
                        }
                        // now put the cards back into their original position
                        board[position.0][position.1] =
                            board[neighbour_position.0][neighbour_position.1].take();
                        board[neighbour_position.0][neighbour_position.1] = temp_neighbour;
                    }
                }
                FightResult::Tie => {
                    // if more than one neighbours have same values, it's a valid capture
                    if same > 1 {
                        // a same capture starts a Same Chain
                        capture_event(neighbour_position, position, board, true);
                    }
                }
                FightResult::Lose => {}
            }
        }

        // captured event for the defender
        fn capture_event(
            defender_position: Position,
            attacker_position: Position,
            board: &mut [[Option<Card>; 5]; 4],
            combo: bool,
        ) {
            // determine the attacking player
            let attacking_player = board[attacker_position.0][attacker_position.1]
                .as_ref()
                .unwrap()
                .player;

            // capture defender if it is not owned by the attacker already
            if board[defender_position.0][defender_position.1]
                .as_ref()
                .unwrap()
                .player
                != attacking_player
            {
                let defender = board[defender_position.0][defender_position.1]
                    .as_mut()
                    .unwrap();
                // change owner of the captured card
                defender.player = attacking_player;
                // Ravager gets an upgrade upon being captured
                if defender.name == Unit::Ravager {
                    defender.upgrade(1);
                }

                // if this card was captures through Same mechanic, it gets played by it's new owner
                if combo {
                    Card::play(board, defender_position.0, defender_position.1, combo);
                }
            }

            // update attacker
            let attacker = board[attacker_position.0][attacker_position.1]
                .as_mut()
                .unwrap();
            // Ravager gets an upgrade upon capturing
            if attacker.name == Unit::Ravager {
                attacker.upgrade(1);
            }
        }

        // handle the battle with top neighbour
        let top_battle = battle(board, y, x, Direction::Top);
        // handle the battle with right neighbour
        let right_battle = battle(board, y, x, Direction::Right);
        // handle the battle with the bottom neighbour
        let bottom_battle = battle(board, y, x, Direction::Bottom);
        // handle the battle with the left neighbour
        let left_battle = battle(board, y, x, Direction::Left);

        let mut same_count: u8 = 0;

        // if this card was not captured by Same Mechanic, count sames
        if !combo {
            if top_battle.0.is_some() && top_battle.0.unwrap() == FightResult::Tie {
                same_count += 1;
            }
            if right_battle.0.is_some() && right_battle.0.unwrap() == FightResult::Tie {
                same_count += 1;
            }
            if bottom_battle.0.is_some() && bottom_battle.0.unwrap() == FightResult::Tie {
                same_count += 1;
            }
            if left_battle.0.is_some() && left_battle.0.unwrap() == FightResult::Tie {
                same_count += 1;
            }
        }

        // proccess result of top battle
        if top_battle.0.is_some() {
            handle_result(
                top_battle.0.unwrap(),
                same_count,
                (y, x),
                top_battle.1,
                board,
                combo,
                Direction::Top,
                false,
            );
        }
        // proccess result of right battle
        if right_battle.0.is_some() {
            handle_result(
                right_battle.0.unwrap(),
                same_count,
                (y, x),
                right_battle.1,
                board,
                combo,
                Direction::Right,
                false,
            );
        }
        // proccess result of bottom battle
        if bottom_battle.0.is_some() {
            handle_result(
                bottom_battle.0.unwrap(),
                same_count,
                (y, x),
                bottom_battle.1,
                board,
                combo,
                Direction::Bottom,
                false,
            );
        }
        // proccess result of left battle
        if left_battle.0.is_some() {
            handle_result(
                left_battle.0.unwrap(),
                same_count,
                (y, x),
                left_battle.1,
                board,
                combo,
                Direction::Left,
                false,
            );
        }
    }
}
