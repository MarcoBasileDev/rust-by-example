use rusty_engine::prelude::*;

#[derive(Resource)]
struct Empty;

fn main() {
    let mut game = Game::new();

    // setup game here

    game.run(Empty);
}
