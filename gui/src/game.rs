pub mod constants;
mod draw;
mod logic;

use std::collections::HashMap;

use crate::{resources::Resources, widgets::*};
use macroquad::prelude::*;
use sol_lib::{board::BoardState, generator::Puzzle};

#[derive(Default)]
pub struct Game {
    resources: Resources,

    settings: Settings,
    puzzle: Puzzle,
    show_rules: bool,
    game_mode: GameMode,
    game_mode_btns: HashMap<GameMode, WidgetId>,

    window_height: f32,
    window_width: f32,

    widgets: HashMap<WidgetId, Widget>,

    board: WidgetId,

    counter_pieces: WidgetId,
    counter_moves: WidgetId,

    label_pieces: WidgetId,
    label_max_age: WidgetId,
    label_heading: WidgetId,

    btn_rules: WidgetId,
    btn_reset: WidgetId,
    btn_next: WidgetId,
    btn_generate: WidgetId,
}

#[derive(Default)]
pub struct Settings {
    pub volume: f32,
    pub debug: bool,
    pub max_pieces: u32,
    pub max_age: u32,
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

#[derive(Copy, Clone)]
pub enum Command {
    NoInteraction,
    ResetBoard,
    NextPuzzle,
    ToggleRules,
    UpdateBoard(BoardState),
    SwitchGameMode(GameMode),
    UpdateMaxPieceCount(u32),
    UpdateMaxAge(u32),
    NewCustomPuzzle,
    ToggleDebug,
    HideRules,
    Quit,
}
