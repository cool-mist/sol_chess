pub mod constants;
mod draw;
mod logic;

use std::collections::HashMap;

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
    id_text_btn: ButtonWidget,
    reset_btn: ButtonWidget,
    next_btn: ButtonWidget,

    show_rules: bool,
    rules_font_size: f32,
    board: BoardWidget,

    heading_text: String,
    heading: LabelWidget,

    rules_btn_text: String,
    rules_btn: ButtonWidget,

    game_mode: GameMode,
    game_mode_btns: HashMap<GameMode, ButtonWidget>,

    pieces_label: LabelWidget,
    max_age_label: LabelWidget,
    pieces_counter: CounterWidget,
    max_age_counter: CounterWidget,
    generate_btn: ButtonWidget,
}

#[derive(Default)]
pub struct GameSettings {
    pub volume: f32,
    pub max_moves_per_piece: u32,
    pub debug: bool,
    pub num_pieces: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameMode {
    Easy,
    Medium,
    Hard,
    Custom,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Medium
    }
}
