pub struct Game {
    pub board: Board,
    pub current_player: usize,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Game {
            board: Board { player1, player2 },
            current_player: 0,
        }
    }

    pub fn play_card(&mut self, card_index: usize) {
        // Implement logic for playing a card
    }

    pub fn attack(&mut self, attacker: usize, target: usize) {
        // Implement logic for attacking
    }

    pub fn use_hero_power(&mut self) {
        // Implement logic for using hero power
    }

    pub fn end_turn(&mut self) {
        // Implement logic for ending the turn
    }

    pub fn perform_action(&mut self, action: Action) {
        // Implement logic for performing actions
    }
}
