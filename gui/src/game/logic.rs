use super::{Game, GameMode, GameSettings, constants};
use crate::{
    resources::{self, *},
    widgets::*,
};
use macroquad::prelude::*;
use sol_lib::{
    board::BoardState,
    generator::{self, Puzzle, RandomRange},
};

impl Game {
    pub async fn initialize_state() -> Self {
        let resources = resources::init().await;
        let game_mode = GameMode::Medium;
        let settings = GameSettings {
            volume: constants::VOLUME,
            ..Default::default()
        };
        let puzzle = Game::generate_puzzle(game_mode);
        let board = BoardWidget::initialize_state(puzzle.board.size, puzzle.board.clone());
        let medium_btn = ButtonWidget::initialize_state(false);
        let next_btn = ButtonWidget::initialize_state(false);
        Self {
            puzzle,
            board,
            resources,
            game_mode,
            settings,
            heading_text: constants::HEADING_TEXT.to_string(),
            rules_text: constants::RULES_BUTTON_TEXT.to_string(),
            medium_btn,
            next_btn,
            ..Self::default()
        }
    }
    pub fn handle_input(&mut self) {
        // CORE GAMEPLAY
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Circle::new(mouse_x, mouse_y, 0.0);
        let board_interaction = self
            .board
            .handle_input(mouse_pos, &self.resources, &self.settings);
        if board_interaction.interacted {
            let Some(board_state) = board_interaction.board_state else {
                return;
            };

            match board_state {
                BoardState::Won => {
                    self.next_btn.is_active = true;
                }
                _ => {}
            }

            return;
        }

        if self
            .reset_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Button),
                self.settings.volume,
            )
            .is_clicked
        {
            self.reset();
            return;
        }

        if self
            .next_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Button),
                self.settings.volume,
            )
            .is_clicked
        {
            self.next_puzzle();
            return;
        }

        // PRESETS
        let mut clicked_game_mode_btn: Option<&ButtonWidget> = None;
        if self
            .easy_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Mode),
                self.settings.volume,
            )
            .is_clicked
        {
            self.game_mode = GameMode::Easy;
            self.easy_btn.is_active = false;
            self.medium_btn.is_active = true;
            self.hard_btn.is_active = true;
            clicked_game_mode_btn = Some(&self.easy_btn);
        } else if self
            .medium_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Mode),
                self.settings.volume,
            )
            .is_clicked
        {
            self.game_mode = GameMode::Medium;
            self.easy_btn.is_active = true;
            self.medium_btn.is_active = false;
            self.hard_btn.is_active = true;
            clicked_game_mode_btn = Some(&self.medium_btn);
        } else if self
            .hard_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Mode),
                self.settings.volume,
            )
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

        if self
            .rules_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Button),
                self.settings.volume,
            )
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
                play_sound_once(
                    &self.resources.sound(&SoundKind::Button),
                    self.settings.volume,
                );
                self.show_rules = false;
                self.rules_text = constants::RULES_BUTTON_TEXT.to_string();
            }

            return;
        }

        if is_key_released(KeyCode::D) {
            self.settings.debug = !self.settings.debug;
            return;
        }

        if is_key_released(KeyCode::Q) {
            std::process::exit(0);
        }
    }

    fn reset(&mut self) {
        self.reset_game(Default::default());
    }

    fn reset_game(&mut self, options: ResetOptions) {
        if options.create_new_puzzle {
            self.puzzle = Game::generate_puzzle(self.game_mode);
        }

        self.next_btn.is_active = false;
        self.board.reset(&self.puzzle);
    }

    fn next_puzzle(&mut self) {
        self.reset_game(ResetOptions {
            create_new_puzzle: true,
            ..Default::default()
        });
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
}

#[derive(Default)]
struct ResetOptions {
    create_new_puzzle: bool,
}

struct MacroquadRandAdapter;
impl RandomRange for MacroquadRandAdapter {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        rand::gen_range(min, max)
    }
}
