#[derive(Debug)]
pub enum Card {
    Minion(Minion),
    Spell(Spell),
}

#[derive(Debug)]
pub struct Minion {
    pub name: String,
    pub cost: u32,
    pub attack: u32,
    pub health: u32,
}

#[derive(Debug)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub effect: Effect,
}

#[derive(Debug)]
pub enum Effect {
    Damage(u32),
    Heal(u32),
}
