use rand::prelude::*;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    target_index: i32,
    // enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            target_index: 0,
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::<GameState>::new();

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.scale = 1.0;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // handle collisions
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // remove the sprite the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
        }
        game_state.score += 1;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = format!("Score: {}", game_state.score);
        if game_state.score > game_state.high_score {
            game_state.high_score = game_state.score;
            let high_score = engine.texts.get_mut("high_score").unwrap();
            high_score.value = format!("High score: {}", game_state.high_score);
        }
        engine.audio_manager.play_sfx(SfxPreset::Minimize1, 1.0);

        println!("Current score: {}", game_state.score);
    }

    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 200.0;

    if engine.keyboard_state.pressed(KeyCode::Up) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed(KeyCode::Right) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed(KeyCode::Left) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("zebra_{}", game_state.target_index);
            game_state.target_index += 1;
            let zebra = engine.add_sprite(label.clone(), "sprite/custom/zebra.png");
            zebra.translation = mouse_location;
            zebra.collision = true;
            zebra.scale = 0.1;
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("zebra_{}", game_state.target_index);
        game_state.target_index += 1;
        let zebra = engine.add_sprite(label.clone(), "sprite/custom/zebra.png");
        zebra.translation.x = thread_rng().gen_range(-550.0..550.0);
        zebra.translation.y = thread_rng().gen_range(-325.0..325.0);
        zebra.collision = true;
        zebra.scale = 0.1;
    }

    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }
}
