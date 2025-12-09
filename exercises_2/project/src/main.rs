use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_left: i32,
}

fn main() {
    let mut game = Game::new();

    // setup game here

    game.run(GameState { health_left: 42 });
}
