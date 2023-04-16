mod card;
mod hero;
mod board;
mod action;
mod game;

use game::Game;
use board::{Player, Board};
use hero::Hero;
use card::{Card, Minion, Spell, Effect};
use action::Action;

fn create_deck() -> Vec<Card> {
    // Create a deck with 30 cards for testing purposes
    vec![]
}

fn main() {
    let player1_deck = create_deck();
    let player2_deck = create_deck();

    let player1_hero = Hero {
        name: String::from("Player 1 Hero"),
        health: 30,
        attack: 2,
    };

    let player2_hero = Hero {
        name: String::from("Player 2 Hero"),
        health: 30,
        attack: 1,
    };

    let player1 = Player {
        hero: player1_hero,
        hand: vec![],
        deck: player1_deck,
        max_mana: 0,
        current_mana: 0,
        minions: vec![],
    };
    
    let player2 = Player {
        hero: player2_hero,
        hand: vec![],
        deck: player2_deck,
        max_mana: 0,
        current_mana: 0,
        minions: vec![],
    };
    

    let mut game = Game::new(player1, player2);

    // Basic game loop
    loop {
        // Display game state
        println!("Current game state: {:#?}", game);

        // Get input for the current player's action
        let action = get_player_action(&game);

        // Perform the action and update the game state
        game.perform_action(action);

        // Check for win conditions and end the game if necessary
        if game_is_over(&game) {
            break;
        }

        // Switch to the next player
        game.end_turn();
    }

    // Announce the winner and end the game
    println!("Game over! The winner is: {}", get_winner(&game));
}

fn get_player_action(game: &Game) -> Action {
    // In a complete implementation, this function should prompt the user for input and validate it
    // For now, we'll return a dummy action
    Action::EndTurn
}

fn game_is_over(game: &Game) -> bool {
    // Check for win conditions, e.g. if a hero's health reaches 0
    game.board.player1.hero.health == 0 || game.board.player2.hero.health == 0
}

fn get_winner(game: &Game) -> &str {
    if game.board.player1.hero.health == 0 {
        "Player 2"
    } else {
        "Player 1"
    }
}
