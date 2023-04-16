pub enum Action {
    PlayCard(usize),
    AttackWithMinion(usize, usize),
    AttackWithHero(usize),
    EndTurn,
}
