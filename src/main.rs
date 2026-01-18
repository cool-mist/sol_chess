use game::{sound::Sounds, Game};
use macroquad::{audio, prelude::*};
use miniquad::date;

mod game;

fn window_conf() -> Conf {
    Conf {
        window_title: get_window_title(),
        fullscreen: false,
        ..Default::default()
    }
}

#[cfg(debug_assertions)]
fn get_window_title() -> String {
    String::from("MOVE TO WORKSPACE 10")
}

#[cfg(not(debug_assertions))]
fn get_window_title() -> String {
    String::from("Solitaire Chess")
}

#[macroquad::main(window_conf)]
async fn main() {
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

    Game::new(texture_res, sounds, font)
}
