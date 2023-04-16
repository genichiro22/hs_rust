use crate::board::Board;
use crate::board::Player;
use crate::action::Action;
use crate::card::Card;
use crate::card::Effect;
use crate::card_samples::sample_cards;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub current_player: u8,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            current_player: 0,
            board: Board {
                player1,
                player2,
            },
        }
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        if self.current_player == 0 {
            &mut self.board.player1
        } else {
            &mut self.board.player2
        }
    }

    pub fn play_card(&mut self, card_index: usize) {
        let current_player = self.get_current_player_mut();
        let card = current_player.hand.remove(card_index);
        let (played_card, effect_to_apply) = match card {
            Card::Minion(minion) => {
                if current_player.current_mana >= minion.cost {
                    current_player.current_mana -= minion.cost;
                    current_player.minions.push(minion);
                    (None, None) // Set played_card to None, and effect_to_apply to None since it's not a spell
                } else {
                    (Some(Card::Minion(minion)), None) // Keep the card in the played_card variable, and effect_to_apply to None
                }
            }
            Card::Spell(spell) => {
                if current_player.current_mana >= spell.cost {
                    current_player.current_mana -= spell.cost;
                    (None, Some(spell.effect)) // Set played_card to None since the card has been played, and effect_to_apply to spell.effect
                } else {
                    (Some(Card::Spell(spell)), None) // Keep the card in the played_card variable, and effect_to_apply to None
                }
            }
        };
    
        // If the card was not played, insert it back into the hand
        if let Some(card) = played_card {
            current_player.hand.insert(card_index, card);
        }
    
        // If there's a spell effect to apply, call apply_spell_effect() here
        if let Some(effect) = effect_to_apply {
            self.apply_spell_effect(&effect);
        }
    }
    
    
    pub fn apply_spell_effect(&mut self, effect: &Effect) {
        match effect {
            Effect::Damage(amount) => {
                // Implement logic for applying damage
            }
            Effect::Heal(amount) => {
                // Implement logic for healing
            }
            Effect::DealDamageToAllEnemyMinions(amount) => {
                // Implement logic for DealDamageToAllEnemyMinions
            }
            Effect::DealDamageToTarget(amount) => {
                // Implement logic for DealDamageToTarget
            }
        }
    }
    
    pub fn attack_with_minion(&mut self, attacker_index: usize, target_index: usize) {
        let (attacker_player, defender_player) = if self.current_player == 0 {
            (&mut self.board.player1, &mut self.board.player2)
        } else {
            (&mut self.board.player2, &mut self.board.player1)
        };
    
        let attacker = &mut attacker_player.minions[attacker_index];
        let defender = &mut defender_player.minions[target_index];
    
        attacker.health = attacker.health.saturating_sub(defender.attack);
        defender.health = defender.health.saturating_sub(attacker.attack);
    
        // Remove minions with 0 or less health
        attacker_player.minions.retain(|m| m.health > 0);
        defender_player.minions.retain(|m| m.health > 0);
    }
    
    pub fn attack_with_hero(&mut self, target_index: usize) {
        let (attacker_player, defender_player) = if self.current_player == 0 {
            (&mut self.board.player1, &mut self.board.player2)
        } else {
            (&mut self.board.player2, &mut self.board.player1)
        };
    
        let attacker = &mut attacker_player.hero;
        let defender = &mut defender_player.minions[target_index];
    
        defender.health = defender.health.saturating_sub(attacker.attack);
    
        // Remove minions with 0 or less health
        defender_player.minions.retain(|m| m.health > 0);
    }

    pub fn end_turn(&mut self) {
        // Increment max_mana, up to a maximum of 10
        let current_player = self.get_current_player_mut();
        if current_player.max_mana < 10 {
            current_player.max_mana += 1;
        }
        // Refill current_mana to max_mana at the end of the turn
        current_player.current_mana = current_player.max_mana;
    
        // Switch to the other player
        self.current_player = 1 - self.current_player;
    }

    pub fn perform_action(&mut self, action: Action) {
        match action {
            Action::PlayCard(card_index) => self.play_card(card_index),
            Action::AttackWithMinion(attacker_index, target_index) => {
                self.attack_with_minion(attacker_index, target_index)
            }
            Action::AttackWithHero(target_index) => self.attack_with_hero(target_index),
            Action::EndTurn => self.end_turn(),
        }
    }
    
}
