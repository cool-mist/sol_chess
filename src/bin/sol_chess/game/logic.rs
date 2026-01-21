use std::{collections::HashMap, rc::Rc};

use super::{
    constants, sound::Sounds, BoardState, ButtonAction, Game, GameMode, GameSquare, GameState,
};

use macroquad::prelude::*;
use sol_chess::{
    board,
    generator::{self, Puzzle, RandomRange},
};

impl Game {
    fn get(&mut self, i: usize, j: usize) -> &mut GameSquare {
        &mut self.squares[i * self.num_squares + j]
    }

    pub fn handle_input(&mut self) {
        let mut gp_btn_clicked = None;
        for btn in &mut self.gp_btns {
            btn.1.handle_input();
            if btn.1.is_clicked() {
                gp_btn_clicked = Some(btn.0.clone());
                break;
            }
        }

        if let Some(action) = gp_btn_clicked {
            match action {
                ButtonAction::Reset => self.reset(),
                ButtonAction::Next => self.next_puzzle(),
            }
        } else {
            let mut mode_btn_clicked = None;
            for btn in &mut self.mode_btns {
                btn.1.handle_input();
                if btn.1.is_clicked() {
                    mode_btn_clicked = Some(btn);
                    break;
                }
            }

            if let Some(btn) = mode_btn_clicked {
                self.game_mode = *btn.0;
                self.next_puzzle();
            } else {
                let mut rules_btn_clicked = false;
                if let Some(btn) = &mut self.rules_btn {
                    btn.handle_input();
                    if btn.is_clicked() {
                        rules_btn_clicked = true;
                    }
                }

                if rules_btn_clicked {
                    self.rules = !self.rules;
                }
            }
        }

        for btn in &mut self.mode_btns {
            if self.game_mode == *btn.0 {
                btn.1.is_active = false;
            } else {
                btn.1.is_active = true;
            }
        }

        if is_key_released(KeyCode::Escape) {
            if self.rules {
                Sounds::play(&self.sounds.button);
            }

            self.rules = false;
        }

        if let Some(rules_btn) = &mut self.rules_btn {
            if self.rules {
                rules_btn.text = constants::RULES_BUTTON_ALT_TEXT.to_string();
            } else {
                rules_btn.text = constants::RULES_BUTTON_TEXT.to_string();
            }
        }

        if is_key_released(KeyCode::D) {
            self.debug = !self.debug;
            return;
        }

        if is_key_released(KeyCode::Q) {
            std::process::exit(0);
        }

        if is_mouse_button_released(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_position(), previous_target)
                }
                GameState::SelectTarget(source) => {
                    let next = self.handle_select_target(mouse_position(), source);
                    if let GameState::SelectTarget(_) = next {
                        self.reset_squares();
                        GameState::SelectSource(None)
                    } else {
                        next
                    }
                }

                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };
            self.state = new_state;
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_position(), previous_target)
                }
                GameState::SelectTarget(source) => GameState::SelectTarget(source),
                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };

            self.state = new_state;
        }
    }

    fn handle_select_source(
        &mut self,
        mouse_position: (f32, f32),
        previous_target: Option<(usize, usize)>,
    ) -> GameState {
        self.reset_squares();
        let (x, y) = mouse_position;
        let mouse = Circle::new(x, y, 0.0);
        let mut selected = None;
        for square in &mut self.squares {
            if mouse.overlaps_rect(&square.rect) {
                if let Some(_) = self.current_board.cells[square.i][square.j] {
                    selected = Some((square.i, square.j));
                }
            }
        }

        if let Some((i, j)) = selected {
            self.get(i, j).is_source = true;
            let mut target_squares = vec![];
            for m in self.current_board.legal_moves.iter() {
                if m.from.file == i && m.from.rank == j {
                    target_squares.push((m.to.file, m.to.rank));
                }
            }

            for (i, j) in target_squares {
                self.get(i, j).is_target = true;
            }

            return GameState::SelectTarget(selected.unwrap());
        }

        if let Some((i, j)) = previous_target {
            self.get(i, j).is_previous_target = true;
        }

        return GameState::SelectSource(None);
    }

    fn handle_select_target(
        &mut self,
        mouse_position: (f32, f32),
        source: (usize, usize),
    ) -> GameState {
        let (x, y) = mouse_position;
        let mouse = Circle::new(x, y, 0.0);

        let mut selected = None;
        for square in &mut self.squares {
            if mouse.overlaps_rect(&square.rect) {
                if let Some(_) = self.current_board.cells[square.i][square.j] {
                    selected = Some((square.i, square.j));
                }
            }
        }

        let (s_x, s_y) = source;
        let Some((x, y)) = selected else {
            self.get(s_x, s_y).is_source = true;
            return GameState::SelectTarget(source);
        };

        if x == s_x && y == s_y {
            self.get(s_x, s_y).is_source = true;
            return GameState::SelectTarget(source);
        }

        let mut is_legal = false;
        if self.get(x, y).is_target {
            is_legal = true;
        }

        if is_legal {
            let m = self.current_board.legal_moves.iter().find(|m| {
                m.from.file == s_x && m.from.rank == s_y && m.to.file == x && m.to.rank == y
            });

            let m = m.expect("legal move should be found");
            self.current_board.make_move(m.clone());

            if self.current_board.game_state == BoardState::Won
                || self.current_board.game_state == BoardState::Lost
            {
                self.reset_squares();
                if self.current_board.game_state == BoardState::Won {
                    let next_btn = self
                        .gp_btns
                        .get_mut(&ButtonAction::Next)
                        .expect("Cannot find next button");
                    next_btn.is_active = true;
                    Sounds::play(&self.sounds.win);
                } else {
                    Sounds::play(&self.sounds.loss);
                }

                return GameState::GameOver((x, y));
            }

            self.reset_squares();
            self.get(x, y).is_target = true;
            Sounds::play(&self.sounds.click);
            return GameState::SelectSource(Some((x, y)));
        }

        self.reset_squares();
        return GameState::SelectSource(None);
    }

    fn reset(&mut self) {
        self.current_board = self.puzzle.board.clone();
        self.reset_squares();

        let next_button = self
            .gp_btns
            .get_mut(&ButtonAction::Next)
            .expect("Cannot find next button");
        next_button.is_active = false;

        self.state = GameState::SelectSource(None);
    }

    fn next_puzzle(&mut self) {
        self.reset();
        let puzzle = Game::generate_puzzle(self.game_mode);
        self.current_board = puzzle.board.clone();
        self.puzzle = puzzle;
    }

    fn reset_squares(&mut self) {
        for i in 0..self.num_squares {
            for j in 0..self.num_squares {
                self.get(i, j).is_source = false;
                self.get(i, j).is_target = false;
            }
        }
    }

    fn generate_puzzle(mode: GameMode) -> Puzzle {
        let piece_count = match mode {
            GameMode::Easy => 3,
            GameMode::Medium => 5,
            GameMode::Hard => 7,
        };

        let generated = generator::generate(piece_count, 100, &MacroquadRandAdapter);
        let puzzle = generated.puzzle();
        puzzle.expect("No puzzle was generated")
    }

    pub fn new_game(texture_res: Texture2D, sounds: Sounds, font: Font) -> Self {
        let num_squares: usize = board::constants::BOARD_SIZE;
        let game_mode = GameMode::Medium;
        let puzzle = Game::generate_puzzle(game_mode);
        let current_board = puzzle.board.clone();

        Self {
            puzzle,
            current_board,
            board_rect: Rect::new(0., 0., 0., 0.),
            squares: Vec::new(),
            heading_rect: Rect::new(0., 0., 0., 0.),
            heading_text: constants::HEADING_TEXT.to_string(),
            heading_font_size: 0.,
            num_squares,
            texture_res,
            sounds,
            state: GameState::SelectSource(None),
            game_mode,
            debug: false,
            gp_btns: HashMap::new(),
            mode_btns: HashMap::new(),
            rules: false,
            rules_btn: None,
            window_height: 0.,
            window_width: 0.,
            square_width: 0.,
            font: Rc::new(font),
        }
    }
}

pub struct MacroquadRandAdapter;
impl RandomRange for MacroquadRandAdapter {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        rand::gen_range(min, max)
    }
}
