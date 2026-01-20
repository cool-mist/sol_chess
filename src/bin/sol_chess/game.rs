use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    sync::Arc,
};

mod button;
mod color;
pub mod constants;
mod draw;
mod logic;
mod shadow;
pub mod sound;
mod texture;

use button::Button;
use macroquad::prelude::*;
use sol_chess::board::{Board, BoardState};
use sound::Sounds;

pub struct Game {
    // The generated puzzle. We keep a copy of this to reset the game.
    original_board: Board,

    // What is shown to the user
    board: Board,

    // Constants througout the game
    texture_res: Texture2D,
    sounds: Sounds,
    font: Arc<Font>,
    num_squares: usize,
    heading_text: String,

    // Update below on handle input
    state: GameState,
    debug: bool,
    game_mode: GameMode,

    // Update below on window resize
    // Used for drawing the state
    square_width: f32,
    window_height: f32,
    window_width: f32,
    board_rect: Rect,
    squares: Vec<GameSquare>,
    heading_rect: Rect,
    heading_font_size: f32,
    gp_btns: HashMap<ButtonAction, Button>,
    mode_btns: HashMap<GameMode, Button>,
    rules: bool,
    rules_btn: Option<Button>,
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
pub enum ButtonAction {
    Reset,
    Next,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameMode {
    Easy,
    Medium,
    Hard,
}

#[derive(Copy, Clone)]
enum GameState {
    SelectSource(Option<(usize, usize)>),
    SelectTarget((usize, usize)),
    GameOver((usize, usize)),
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
