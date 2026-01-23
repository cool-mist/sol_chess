pub mod constants;
mod draw;
mod logic;

use crate::{
    resources::Resources,
    widgets::{button::Button, id_text_input::IdTextInput},
};
use macroquad::prelude::*;
use sol_lib::{board::Board, generator::Puzzle};
use std::fmt::{self, Display, Formatter};

#[derive(Default)]
pub struct Game {
    resources: Resources,

    volume: f32,

    puzzle: Puzzle,
    state: GameState,
    debug: bool,

    window_height: f32,
    window_width: f32,

    current_board: Board,
    square_width: f32,
    num_squares: usize, // this is a constant = 4 for now.
    board_rect: Rect,
    squares: Vec<GameSquare>,

    heading_rect: Rect,
    heading_font_size: f32,
    heading_text: String,

    show_rules: bool,
    rules_text: String,
    rules_btn: Button,

    id_text: IdTextInput,

    reset_btn: Button,

    next_btn: Button,

    game_mode: GameMode,
    easy_btn: Button,
    medium_btn: Button,
    hard_btn: Button,
}

struct GameSquare {
    rect: Rect,
    color: Color,
    is_source: bool,
    is_target: bool,
    is_previous_target: bool,
    i: usize,
    j: usize,
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

#[derive(Copy, Clone)]
enum GameState {
    SelectSource(Option<(usize, usize)>),
    SelectTarget((usize, usize)),
    GameOver((usize, usize)),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::SelectSource(None)
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameState::SelectSource(Some(x)) => write!(f, "Select Source [ {}, {} ]", x.0, x.1),
            GameState::SelectSource(None) => write!(f, "Select Source [ ]"),
            GameState::SelectTarget(x) => write!(f, "Select Target [ {}, {} ]", x.0, x.1),
            GameState::GameOver(x) => write!(f, "Game Over [ {}, {} ]", x.0, x.1),
        }
    }
}
