use std::{
    cmp::{max, min},
    mem::swap,
    ops::Range,
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

#[derive(Debug)]
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

    // adds a card stack (4 cards) of the specified type to the deck
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

    // pulls neighbours
    pub fn pull(
        board: &mut [[Option<Card>; 5]; 4],
        position: Position,
        neighbours: &mut [Option<Position>; 4],
        bombs: &mut [[u8; 5]; 4],
    ) {
        let (y, x) = position;
        let directions = [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ];

        for i in 0..4 {
            // if neighbour exists
            if let Some(neighbour) = neighbours[i] {
                let (ny, nx) = neighbour;
                let direction = directions[i];

                // calculate damage of inbetween bombs
                let mut damage: u8 = 0;
                let iter: Range<usize> = match direction {
                    Direction::Top => ny + 1..y,
                    Direction::Right => x + 1..nx,
                    Direction::Bottom => y + 1..ny,
                    Direction::Left => nx + 1..x,
                };
                for i in iter {
                    match direction {
                        Direction::Top | Direction::Bottom => {
                            damage += bombs[i][x];
                            bombs[i][x] = 0;
                        }
                        Direction::Right | Direction::Left => {
                            damage += bombs[y][i];
                            bombs[y][i] = 0;
                        }
                    }
                }

                // fetch the card that needs to be pulled
                let mut neighbour = board[ny][nx].take();

                // apply damage
                neighbour.as_mut().unwrap().downgrade(damage);

                // relocate card
                match direction {
                    Direction::Top => {
                        board[y - 1][x] = neighbour;
                        neighbours[i] = Some((y - 1, x));
                    }
                    Direction::Right => {
                        board[y][x + 1] = neighbour;
                        neighbours[i] = Some((y, x + 1));
                    },
                    Direction::Bottom => {
                        board[y + 1][x] = neighbour;
                        neighbours[i] = Some((y + 1, x));
                    },
                    Direction::Left => { 
                        board[y][x - 1] = neighbour;
                        neighbours[i] = Some((y, x - 1));
                    },
                }
            }
        }
    }

    // flip the facing value with the value of the other end
    pub fn flip(board: &mut [[Option<Card>; 5]; 4], neighbours: [Option<Position>; 4]) {
        for i in 0..4 {
            if let Some(neighbour) = neighbours[i] {
                let n = board[neighbour.0][neighbour.1].as_mut().unwrap();
                match i {
                    // top & bottom
                    0 | 2 => swap(&mut n.top, &mut n.bottom),
                    // left & right
                    1 | 3 => swap(&mut n.right, &mut n.left),
                    _ => {}
                }
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

    // gets the first neighbour in specified direction (for Siren & Keeper)
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

    // returns neigbour with it's position
    fn get_neighbour(
        board: &[[Option<Card>; 5]; 4],
        direction: Direction,
        y: usize,
        x: usize,
        card: Unit,
    ) -> Option<Position> {
        match direction {
            Direction::Top => {
                if y > 0 {
                    let n: Option<usize> = match card {
                        // Keeper and Siren are ranged
                        Unit::Keeper | Unit::Siren => {
                            Card::get_far_neighbour(direction, board, y, x)
                        }
                        // Others are melee
                        _ => {
                            if board[y - 1][x].is_some() {
                                Some(y - 1)
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(np) = n {
                        return Some((np, x));
                    }
                }

                return None;
            }
            Direction::Right => {
                if x < 4 {
                    let n: Option<usize> = match card {
                        // Keeper and Siren are ranged
                        Unit::Keeper | Unit::Siren => {
                            Card::get_far_neighbour(direction, board, y, x)
                        }
                        // Others are melee
                        _ => {
                            if board[y][x + 1].is_some() {
                                Some(x + 1)
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(np) = n {
                        return Some((y, np));
                    }
                }

                return None;
            }
            Direction::Bottom => {
                if y < 3 {
                    let n: Option<usize> = match card {
                        // Keeper and Siren are ranged
                        Unit::Keeper | Unit::Siren => {
                            Card::get_far_neighbour(direction, board, y, x)
                        }
                        // Others are melee
                        _ => {
                            if board[y + 1][x].is_some() {
                                Some(y + 1)
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(np) = n {
                        return Some((np, x));
                    }
                }

                return None;
            }
            Direction::Left => {
                if x > 0 {
                    let n: Option<usize> = match card {
                        // Keeper and Siren are ranged
                        Unit::Keeper | Unit::Siren => {
                            Card::get_far_neighbour(direction, board, y, x)
                        }
                        // Others are melee
                        _ => {
                            if board[y][x - 1].is_some() {
                                Some(x - 1)
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(np) = n {
                        return Some((y, np));
                    }
                }

                return None;
            }
        }
    }

    // returns Top, Right, Bottom, Left neighbours in an array
    pub fn get_neighbours(
        board: &[[Option<Card>; 5]; 4],
        y: usize,
        x: usize,
        card: Unit,
    ) -> [Option<Position>; 4] {
        let mut neighbours: [Option<Position>; 4] = Default::default();
        let dirs = [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ];

        for i in 0..4 {
            neighbours[i] = Card::get_neighbour(board, dirs[i], y, x, card);
        }

        return neighbours;
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
        neighbours: &mut [Option<Position>; 4],
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

                Card::placement(board, mov, bombs, neighbours);

                Card::play(board, mov.0, mov.1, false, Some(*neighbours));

                return true;
            }
        }
    }

    // placement event that needs to be run after card is placed on the board for the first time
    pub fn placement(
        board: &mut [[Option<Card>; 5]; 4],
        position: Position,
        bombs: &mut [[u8; 5]; 4],
        neighbours: &mut [Option<Position>; 4],
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
                Card::pull(board, position, neighbours, bombs);
            }
            // Titan flips adjacent cards
            Unit::Titan => {
                Card::flip(board, *neighbours);
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

    // play event that need to be run when the card is already placed on the board
    pub fn play(
        board: &mut [[Option<Card>; 5]; 4],
        y: usize,
        x: usize,
        combo: bool,
        neighbours: Option<[Option<Position>; 4]>,
    ) {
        // result of the fight between two cards
        #[derive(Clone, Copy, PartialEq, Debug)]
        enum FightResult {
            Win,
            Tie,
            Lose,
        }

        // handles battle with specified neighbour and returns the result
        fn battle(
            board: &mut [[Option<Card>; 5]; 4],
            position: Position,
            neighbour_position: Position,
            direction: Direction,
        ) -> Option<FightResult> {
            let (y, x) = position;
            let (ny, nx) = neighbour_position;
            // determine the attacker
            let attacking_player: u8 = board[y][x].as_ref().unwrap().player;
            let neighbour = board[ny][nx].as_ref();

            // if neighbour exists, check it
            if let Some(neighbour) = neighbour {
                // fetch attacker
                let a = board[y][x].as_ref().unwrap();
                // fetch defender
                let d = neighbour;

                match direction {
                    Direction::Top => {
                        return Some(fight(
                            a.name,
                            d.name,
                            a.top,
                            d.bottom,
                            board,
                            attacking_player,
                            d.player,
                        ));
                    }
                    Direction::Right => {
                        return Some(fight(
                            a.name,
                            d.name,
                            a.right,
                            d.left,
                            board,
                            attacking_player,
                            d.player,
                        ));
                    }
                    Direction::Bottom => {
                        return Some(fight(
                            a.name,
                            d.name,
                            a.bottom,
                            d.top,
                            board,
                            attacking_player,
                            d.player,
                        ));
                    }
                    Direction::Left => {
                        return Some(fight(
                            a.name,
                            d.name,
                            a.left,
                            d.right,
                            board,
                            attacking_player,
                            d.player,
                        ));
                    }
                }
            }

            // if neighbour is empty or no fight was fought, return None
            return None;
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

                    let (y, x) = position;
                    let (ny, nx) = neighbour_position;

                    // Lancer has pierce ability which attacks fallen neighbour's neighbour
                    if !pierce && board[y][x].as_ref().unwrap().name == Unit::Lancer {
                        // fetch defender (card to be pierced)
                        let def = Card::get_neighbour(board, direction, ny, nx, Unit::Lancer);

                        // if there is a card to pierce
                        if let Some(d) = def {
                            // swap lancer with it's defeated neighbour
                            let temp_card = board[ny][nx].take();
                            board[ny][nx] = board[y][x].take();

                            // commence battle at the neighbour's position
                            let battle_result = battle(board, neighbour_position, d, direction);

                            // check if there was a battle
                            if battle_result.is_some() {
                                // handle the result
                                handle_result(
                                    battle_result.unwrap(),
                                    0,
                                    neighbour_position,
                                    d,
                                    board,
                                    combo,
                                    direction,
                                    true,
                                );
                            }

                            // now put the cards back into their original positions
                            board[y][x] = board[ny][nx].take();
                            board[ny][nx] = temp_card;
                        }
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
                    Card::play(board, defender_position.0, defender_position.1, combo, None);
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

        // fetch neighbours
        let neighbours = match neighbours {
            Some(n) => n,
            None => Card::get_neighbours(board, y, x, board[y][x].as_ref().unwrap().name),
        };

        // println!("All the neighbours ==> {:#?}", neighbours);

        // handle the battle with top neighbour
        let top_battle = match neighbours[0] {
            Some(n) => battle(board, (y, x), n, Direction::Top),
            None => None,
        };
        // handle the battle with right neighbour
        let right_battle = match neighbours[1] {
            Some(n) => battle(board, (y, x), n, Direction::Right),
            None => None,
        };
        // handle the battle with the bottom neighbour
        let bottom_battle = match neighbours[2] {
            Some(n) => battle(board, (y, x), n, Direction::Bottom),
            None => None,
        };
        // handle the battle with the left neighbour
        let left_battle = match neighbours[3] {
            Some(n) => battle(board, (y, x), n, Direction::Left),
            None => None,
        };

        let mut same_count: u8 = 0;

        // if this card was not captured by Same Mechanic, count sames
        if !combo {
            // fn to count Ties
            fn count_same(same_count: &mut u8, fr: FightResult) {
                if fr == FightResult::Tie {
                    *same_count += 1;
                }
            }

            if let Some(res) = top_battle {
                count_same(&mut same_count, res);
            }
            if let Some(res) = right_battle {
                count_same(&mut same_count, res);
            }
            if let Some(res) = bottom_battle {
                count_same(&mut same_count, res);
            }
            if let Some(res) = left_battle {
                count_same(&mut same_count, res);
            }
        }

        // handle result of top battle
        if let Some(res) = top_battle {
            handle_result(
                res,
                same_count,
                (y, x),
                neighbours[0].unwrap(),
                board,
                combo,
                Direction::Top,
                false,
            );
        }
        // handle result of right battle
        if let Some(res) = right_battle {
            handle_result(
                res,
                same_count,
                (y, x),
                neighbours[1].unwrap(),
                board,
                combo,
                Direction::Right,
                false,
            );
        }
        // handle result of bottom battle
        if let Some(res) = bottom_battle {
            handle_result(
                res,
                same_count,
                (y, x),
                neighbours[2].unwrap(),
                board,
                combo,
                Direction::Bottom,
                false,
            );
        }
        // handle result of left battle
        if let Some(res) = left_battle {
            handle_result(
                res,
                same_count,
                (y, x),
                neighbours[3].unwrap(),
                board,
                combo,
                Direction::Left,
                false,
            );
        }
    }
}
