use crate::card::Minion;

#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub health: u32,
    pub attack: u32,
}

impl Hero {
    pub fn attack(&mut self, target: &mut Minion) {
        target.health = target.health.saturating_sub(self.attack);
    }
}