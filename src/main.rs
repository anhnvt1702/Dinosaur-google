//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example road_race

// use std::time::Duration;

use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const ROAD_SPEED: f32 = 600.0;

struct GameState {
    health_amount: u8,
    lost: bool,
    started: bool,
    score: i32,
}

fn main() {
    let mut game = Game::new();

    // Create the player sprite
    let player1 = game.add_sprite("player1", SpritePreset::Dino);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    // Start some background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Create the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
        roadline.translation.y = -25.0;
    }

    // Create the obstacle sprites
    let obstacle = game.add_sprite("obstacle", SpritePreset::Tree);
    obstacle.layer = 5.0;
    obstacle.collision = true;
    obstacle.translation.x = -500.0;

    // Create the health message
    let health_message = game.add_text("health_message", "Health: 3");
    health_message.translation = Vec2::new(550.0, 320.0);

    //Score
    let score_text = game.add_text("score_text", "Score: 0");
    score_text.translation = Vec2::new(550.0, 290.0);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 3,
        lost: false,
        started: false,
        score: 0,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Đặt biến để theo dõi trạng thái nhảy của player
    let mut time_start = 0;
    const JUMP_DURATION: f32 = 3.0;
    let mut jumping = false;
    let mut jump_time = 0.0;
    let mut initial_y = 0.0;
    // Don't run any more game logic if the game has ended
    if !game_state.started && engine.keyboard_state.pressed(KeyCode::Space) {
        time_start = engine.time_since_startup_f64 as i32;
        game_state.started = true; // Bắt đầu trò chơi
    }

    // Không chạy logic trò chơi nếu chưa bắt đầu
    if !game_state.started {
        return;
    }
    if game_state.lost {
        return;
    }
    let time_score = (engine.time_since_startup_f64 as i32) - time_start;
    game_state.score += time_score  ;
    // In điểm ra màn hình
    let score_text = engine.texts.get_mut("score_text").unwrap();
    score_text.value = format!("Score : {}", game_state.score);

    if engine.keyboard_state.pressed(KeyCode::Up) && !jumping {
        // Bắt đầu quá trình nhảy lên
        let player1 = engine.sprites.get_mut("player1").unwrap();
        jumping = true;
        jump_time = 0.0;
        initial_y = player1.translation.y;
    }

    // Khi đang trong giai đoạn nhảy
    if jumping {
        let player1 = engine.sprites.get_mut("player1").unwrap();

        // Tính thời gian đã nhảy
        jump_time += engine.delta_f32;

        // Tính vị trí y của player trong giai đoạn nhảy
        // Sử dụng hàm số parabol để làm cho nhảy không đều
        let jump_height = 1000.0;
        let y_offset = -(4.0 * jump_height / JUMP_DURATION.powi(2)) * jump_time.powi(2)
            + (4.0 * jump_height / JUMP_DURATION) * jump_time;
        player1.translation.y = initial_y + y_offset;

      
    }
    // Khi không ấn Up
    else {
        // Thay đổi giá trị y của player theo hướng xuống (giảm y)
        let player1 = engine.sprites.get_mut("player1").unwrap();
        player1.translation.y -= PLAYER_SPEED * engine.delta_f32;

        // Kiểm tra nếu player đạt đến mặt đất (y <= 0.0)
        if player1.translation.y <= 0.0 {
            // Đặt lại y của player về mặt đất
            player1.translation.y = 0.0;
        }
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(400.0..800.0);
            }
        }
    }

    // Deal with collisions
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
