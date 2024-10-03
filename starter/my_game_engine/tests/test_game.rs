// tests/test_game.rs
use my_game_engine::*; // Ensure that all the necessary functions are available from this module
use std::thread;
use std::time::Duration;

#[test]
fn test_simple_game_loop() {
    create_window(800, 600);
    while !window_should_close() {
        update_game_window();
        thread::sleep(Duration::from_millis(16));
    }
}

#[test]
fn test_sprite_rendering() {
    create_window(800, 600);
    let sprite_id = create_sprite("path/to/sprite.png");
    while !window_should_close() {
        render_sprite(sprite_id);
        update_game_window();
        thread::sleep(Duration::from_millis(16));
    }
}

#[test]
fn test_screen_clearing() {
    create_window(800, 600);
    let red_sprite_id = create_sprite("path/to/red_sprite.png");
    let green_sprite_id = create_sprite("path/to/green_sprite.png");

    let start_time = std::time::Instant::now();
    while !window_should_close() {
        render_sprite(red_sprite_id);
        update_game_window();
        if start_time.elapsed().as_secs() >= 5 {
            clear_screen();
            render_sprite(green_sprite_id);
        }
        thread::sleep(Duration::from_millis(16));
    }
}

#[test]
fn test_key_presses() {
    create_window(800, 600);
    let mut a_pressed = false;
    let mut b_pressed = false;

    while !window_should_close() {
        if get_key("UP") {
            a_pressed = true;
        }
        if get_key("DOWN") {
            b_pressed = true;
        }

        if a_pressed && b_pressed {
            break;
        }

        update_game_window();
        thread::sleep(Duration::from_millis(16));
    }
}

#[test]
fn test_sprite_position_update() {
    create_window(800, 600);
    let sprite_id = create_sprite("path/to/sprite.png");
    let mut position = (0, 300); // Start position

    while !window_should_close() {
        clear_screen();
        update_sprite_position(sprite_id, position.0, position.1);
        render_sprite(sprite_id);
        update_game_window();

        position.0 += 1; 
        if position.0 > 800 {
            break;
        }

        thread::sleep(Duration::from_millis(16));
    }
}
