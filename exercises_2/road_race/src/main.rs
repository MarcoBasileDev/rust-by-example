use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
}
