pub mod constants;
mod draw;
mod logic;

use crate::{
    game::logic::{GameMode, GameState},
    resources::Resources,
    widgets::*,
};
use macroquad::prelude::*;
use sol_lib::{board::Board, generator::Puzzle};

#[derive(Default)]
pub struct Game {
    resources: Resources,

    volume: f32,

    debug: bool,

    window_height: f32,
    window_width: f32,

    puzzle: Puzzle,
    id_text: IdTextInput,
    reset_btn: ButtonWidget,
    next_btn: ButtonWidget,

    current_board: Board,
    square_width: f32,
    num_squares: usize, // this is a constant = 4 for now.
    board_rect: Rect,
    squares: Vec<GameSquare>,
    state: GameState,

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

struct GameSquare {
    rect: Rect,
    color: Color,
    is_source: bool,
    is_target: bool,
    is_previous_target: bool,
    i: usize,
    j: usize,
}
