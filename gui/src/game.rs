pub mod constants;
mod draw;
mod logic;

use crate::{resources::Resources, widgets::*};
use macroquad::prelude::*;
use sol_lib::generator::Puzzle;

#[derive(Default)]
pub struct Game {
    resources: Resources,
    settings: GameSettings,

    window_height: f32,
    window_width: f32,

    puzzle: Puzzle,
    id_text: IdTextInput,
    reset_btn: ButtonWidget,
    next_btn: ButtonWidget,

    board: BoardWidget,

    heading_rect: Rect,
    heading_font_size: f32,
    heading_text: String,

    show_rules: bool,
    rules_text: String,
    rules_btn: ButtonWidget,

    game_mode: GameMode,
    easy_btn: ButtonWidget,
    medium_btn: ButtonWidget,
    hard_btn: ButtonWidget,
}

#[derive(Default)]
pub struct GameSettings {
    pub volume: f32,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameMode {
    Easy,
    Medium,
    Hard,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Medium
    }
}
