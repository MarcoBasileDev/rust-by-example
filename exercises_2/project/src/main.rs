use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    current_score: u32,
    high_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();

    let game_state = GameState {
        current_score: 0,
        high_score: 2345,
        enemy_labels: Vec::new(),
        spawn_timer: Timer::from_seconds(10.0, TimerMode::Once),
    };
    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    game_state.current_score += 1;
    println!("Current score: {}", game_state.current_score);
}
