pub enum Card {
    Minion(Minion),
    Spell(Spell),
    Weapon(Weapon),
}

pub struct Minion {
    pub name: String,
    pub cost: u32,
    pub attack: u32,
    pub health: u32,
    pub abilities: Vec<Ability>,
}

pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub effect: Effect,
}

pub struct Weapon {
    pub name: String,
    pub cost: u32,
    pub attack: u32,
    pub durability: u32,
}

pub enum Ability {
    Taunt,
    Charge,
    Battlecry,
    Deathrattle,
}

pub enum Effect {
    Damage(u32),
    Heal(u32),
    Draw(u32),
    // Add more effects as needed
}
