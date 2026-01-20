mod game;

use game::{sound::Sounds, Game};
use macroquad::{audio, prelude::*};
use miniquad::date;
use sol_chess::board;

use game::constants;

fn window_conf() -> Conf {
    Conf {
        window_title: constants::WINDOW_TITLE.to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if board::constants::BOARD_SIZE != 4 {
        panic!("NUM_SQUARES != 4 is not yet supported");
    }

    rand::srand(date::now() as u64);
    let background_color = Color::from_rgba(196, 195, 208, 255);
    let mut game = init().await;
    loop {
        clear_background(background_color);
        game.handle_input();
        game.draw();
        next_frame().await
    }
}

macro_rules! load_sound {
    ($file_name:expr) => {
        audio::load_sound_from_bytes(include_bytes!($file_name))
            .await
            .unwrap()
    };
}

async fn init() -> Game {
    let texture_bytes = include_bytes!("../assets/pieces.png");
    let texture_res = Texture2D::from_file_with_format(&texture_bytes[..], None);
    texture_res.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    let click = load_sound!("../assets/click.wav");
    let win = load_sound!("../assets/win.wav");
    let loss = load_sound!("../assets/loss.wav");
    let button = load_sound!("../assets/button.wav");
    let mode = load_sound!("../assets/mode.wav");
    let sounds = Sounds {
        click,
        win,
        loss,
        button,
        mode,
    };

    let font_ttf = include_bytes!("../assets/caskaydia.ttf");
    let Ok(font) = load_ttf_font_from_bytes(font_ttf) else {
        panic!("Failed to load font");
    };

    Game::new_game(texture_res, sounds, font)
}
