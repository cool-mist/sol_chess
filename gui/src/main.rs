mod game;
mod resources;
mod widgets;

use game::Game;
use macroquad::prelude::*;
use miniquad::date;

use game::constants;

fn window_conf() -> Conf {
    let window_title = {
        if cfg!(debug_assertions) {
            "Debug: Puzzle Game"
        } else {
            constants::WINDOW_TITLE
        }
    };

    Conf {
        window_title: String::from(window_title),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(date::now() as u64);
    let background_color = Color::from_rgba(196, 195, 208, 255);
    let mut game = Game::new().await;
    loop {
        clear_background(background_color);

        let interaction = game.handle_user_input();
        game.handle_interaction(interaction);

        game.draw();
        next_frame().await
    }
}

