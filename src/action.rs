pub enum Action {
    PlayCard(usize),
    Attack(usize, usize),
    HeroPower,
    EndTurn,
}
