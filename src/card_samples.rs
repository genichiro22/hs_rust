use crate::card::{Card, Minion, Spell};
use crate::card::Effect;

pub fn sample_cards() -> Vec<Card> {
    vec![
        Card::Minion(Minion {
            name: String::from("Murloc Raider"),
            cost: 1,
            attack: 2,
            health: 1,
        }),
        Card::Minion(Minion {
            name: String::from("River Crocolisk"),
            cost: 2,
            attack: 2,
            health: 3,
        }),
        Card::Minion(Minion {
            name: String::from("Boulderfist Ogre"),
            cost: 6,
            attack: 6,
            health: 7,
        }),
        Card::Spell(Spell {
            name: String::from("Arcane Explosion"),
            cost: 2,
            effect: Effect::DealDamageToAllEnemyMinions(1),
        }),
        Card::Spell(Spell {
            name: String::from("Fireball"),
            cost: 4,
            effect: Effect::DealDamageToTarget(6),
        }),
    ]
}
