mod game;
mod resources;
mod widgets;

use std::collections::HashSet;

use macroquad::prelude::*;
use miniquad::date;

use game::constants;

use crate::widgets::Interaction;

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
    let mut game = game::new_game().await;
    loop {
        clear_background(background_color);

        game.update(get_interaction());
        game.draw();

        next_frame().await;
    }
}

fn get_interaction() -> Interaction {
    let mut mouse_pressed = HashSet::new();
    let mut mouse_released = HashSet::new();
    let buttons = vec![MouseButton::Left, MouseButton::Right];
    for btn in buttons {
        if is_mouse_button_pressed(btn) {
            mouse_pressed.insert(btn);
        }

        if is_mouse_button_released(btn) {
            mouse_released.insert(btn);
        }
    }

    let (mx, my) = mouse_position();
    Interaction {
        mouse_pos: Circle::new(mx, my, 0.),
        keys_released: get_keys_released(),
        mouse_pressed,
        mouse_released,
    }
}
