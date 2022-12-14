use gameplay::game::check_collision;
use macroquad::prelude::*;

pub mod gameplay;
pub use crate::gameplay::game;

#[macroquad::main("Sumo")]
async fn main() {
    request_new_screen_size(750.0, 750.0);
    next_frame().await;

    // Creating the players
    let mut player_1 = game::Player::new(
        187.5, 
        375.0,
        10.0,
        RED,
        game::ControlScheme::new(
            KeyCode::W,
            KeyCode::S,
            KeyCode::A,
            KeyCode::D
        )
    );

    let mut player_2 = game::Player::new(
        562.5, 
        375.0,
        10.0,
        BLUE,
        game::ControlScheme::new(
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right
        )
    );

    // The text for displaying who won.
    let mut text_for_winner: String = String::from("");

    loop {
        clear_background(WHITE);

        // Checking and notifying of winner
        game::check_winner(&player_1, &player_2, &mut text_for_winner);
        draw_text(text_for_winner.as_str(), screen_width() * 0.5 - 115.0, screen_height() * 0.075, 40.0, GREEN);

        // Rendering the sumo mat and players onto the screen
        draw_circle(screen_width()/2.0, screen_height()/2.0, 225.0, GRAY);
        player_1.draw();
        player_2.draw();

        // Updating the players
        player_1.update();
        player_2.update();

        check_collision(&mut player_1, &mut player_2);

        if is_key_down(KeyCode::R){
            player_1.reset();
            player_2.reset();

            text_for_winner = String::from("");
        }

        next_frame().await
    }
}
