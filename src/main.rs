
enum Unit {
    Warden,
    Siren,
    Keeper,
    Saboteur,
    Swarm,
    Slayer,
    Titan,
    Ravager
}

struct Card {
    pub name: Unit,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8
}

impl Card {
    fn get_values(&self) -> [u8; 4] {
        match self.name {
            Unit::Warden => [6, 6, 4, 4],
            Unit::Siren => [7, 4, 4, 5],
            Unit::Keeper => [9, 5, 1, 5],
            Unit::Saboteur => [6, 5, 4, 5],
            Unit::Swarm => [7, 3, 3, 3],
            Unit::Slayer => [1, 6, 7, 6],
            Unit::Titan => [7, 4, 6, 3],
            Unit::Ravager => [8, 4, 6, 2]
        }
    }
    
    fn placement() {

    }

    fn attack() {

    }

    fn endturn() {

    }
}


fn main() {

}
