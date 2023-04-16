pub struct Board {
    pub player1: Player,
    pub player2: Player,
}

pub struct Player {
    pub hero: Hero,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
    pub mana: u32,
    pub minions: Vec<Minion>,
}
