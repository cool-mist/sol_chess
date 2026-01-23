use super::{Game, GameMode, GameSquare, GameState, constants};
use crate::{
    resources::{Resources, SoundKind},
    widgets::{button::Button, *},
};
use macroquad::prelude::*;
use sol_lib::{
    board::BoardState,
    generator::{self, Puzzle, RandomRange},
};

impl Game {
    fn get(&mut self, i: usize, j: usize) -> &mut GameSquare {
        &mut self.squares[i * self.num_squares + j]
    }

    pub fn handle_input(&mut self) {
        // CORE GAMEPLAY
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Circle::new(mouse_x, mouse_y, 0.0);
        if is_mouse_button_released(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_pos, previous_target)
                }
                GameState::SelectTarget(source) => {
                    let next = self.handle_select_target(mouse_pos, source);
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
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_pos, previous_target)
                }
                GameState::SelectTarget(source) => GameState::SelectTarget(source),
                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };

            self.state = new_state;
        }

        if (self
            .reset_btn
            .handle_input(&self.resources.sound(&SoundKind::Button), self.volume))
        .is_clicked
        {
            self.reset();
            return;
        }

        if (self
            .next_btn
            .handle_input(&self.resources.sound(&SoundKind::Button), self.volume))
        .is_clicked
        {
            self.next_puzzle();
            return;
        }

        // PRESETS
        let mut clicked_game_mode_btn: Option<&Button> = None;
        if (self
            .easy_btn
            .handle_input(&self.resources.sound(&SoundKind::Mode), self.volume))
        .is_clicked
        {
            self.game_mode = GameMode::Easy;
            self.easy_btn.is_active = false;
            self.medium_btn.is_active = true;
            self.hard_btn.is_active = true;
            clicked_game_mode_btn = Some(&self.easy_btn);
        } else if (self
            .medium_btn
            .handle_input(&self.resources.sound(&SoundKind::Mode), self.volume))
        .is_clicked
        {
            self.game_mode = GameMode::Medium;
            self.easy_btn.is_active = true;
            self.medium_btn.is_active = false;
            self.hard_btn.is_active = true;
            clicked_game_mode_btn = Some(&self.medium_btn);
        } else if (self
            .hard_btn
            .handle_input(&self.resources.sound(&SoundKind::Mode), self.volume))
        .is_clicked
        {
            self.game_mode = GameMode::Hard;
            self.easy_btn.is_active = true;
            self.medium_btn.is_active = true;
            self.hard_btn.is_active = false;
            clicked_game_mode_btn = Some(&self.hard_btn);
        }

        if let Some(_) = clicked_game_mode_btn {
            self.next_puzzle();
            return;
        }

        if (self
            .rules_btn
            .handle_input(&self.resources.sound(&SoundKind::Button), self.volume))
        .is_clicked
        {
            self.show_rules = !self.show_rules;
            self.rules_text = match self.show_rules {
                true => constants::RULES_BUTTON_ALT_TEXT.to_string(),
                false => constants::RULES_BUTTON_TEXT.to_string(),
            };

            return;
        }

        // KEY INPUTS
        if is_key_released(KeyCode::Escape) {
            if self.show_rules {
                play_sound_once(&self.resources.sound(&SoundKind::Button), self.volume);
                self.show_rules = false;
                self.rules_text = constants::RESET_BUTTON_TEXT.to_string();
            }

            return;
        }

        if is_key_released(KeyCode::D) {
            self.debug = !self.debug;
            return;
        }

        if is_key_released(KeyCode::Q) {
            std::process::exit(0);
        }
    }

    fn handle_select_source(
        &mut self,
        mouse_pos: Circle,
        previous_target: Option<(usize, usize)>,
    ) -> GameState {
        self.reset_squares();
        let mut selected = None;
        for square in &mut self.squares {
            if mouse_pos.overlaps_rect(&square.rect) {
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

    fn handle_select_target(&mut self, mouse_pos: Circle, source: (usize, usize)) -> GameState {
        let mut selected = None;
        for square in &mut self.squares {
            if mouse_pos.overlaps_rect(&square.rect) {
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
                    self.next_btn.is_active = true;
                    play_sound_once(&self.resources.sound(&SoundKind::Win), self.volume);
                } else {
                    play_sound_once(&self.resources.sound(&SoundKind::Loss), self.volume);
                }

                return GameState::GameOver((x, y));
            }

            self.reset_squares();
            self.get(x, y).is_target = true;
            play_sound_once(&self.resources.sound(&SoundKind::Click), self.volume);
            return GameState::SelectSource(Some((x, y)));
        }

        self.reset_squares();
        return GameState::SelectSource(None);
    }

    fn reset(&mut self) {
        self.current_board = self.puzzle.board.clone();
        self.next_btn.is_active = false;
        self.state = GameState::SelectSource(None);
        self.reset_squares();
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

        let generated =
            generator::generate_weighted_random(piece_count, 100, &MacroquadRandAdapter);
        let puzzle = generated.puzzle();
        puzzle.expect("No puzzle was generated")
    }

    pub fn new_game(resources: Resources) -> Self {
        let game_mode = GameMode::Medium;
        let puzzle = Game::generate_puzzle(game_mode);
        let current_board = puzzle.board.clone();
        let num_squares: usize = current_board.size;
        Self {
            puzzle,
            current_board,
            resources,
            game_mode,
            num_squares,
            heading_text: constants::HEADING_TEXT.to_string(),
            volume: constants::VOLUME,
            ..Self::default()
        }
    }
}

pub struct MacroquadRandAdapter;
impl RandomRange for MacroquadRandAdapter {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        rand::gen_range(min, max)
    }
}
