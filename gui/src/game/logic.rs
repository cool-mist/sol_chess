use std::collections::HashMap;

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
            max_moves_per_piece: 3,
            num_pieces: 2,
            ..Default::default()
        };
        let puzzle = Game::generate_puzzle(game_mode, &settings);
        let board = BoardWidget::initialize_state(puzzle.board.size, puzzle.board.clone());
        let mut game_mode_btns = HashMap::new();
        let game_mode = GameMode::Medium;
        game_mode_btns.insert(GameMode::Easy, ButtonWidget::initialize_state(true));
        game_mode_btns.insert(GameMode::Medium, ButtonWidget::initialize_state(true));
        game_mode_btns.insert(GameMode::Hard, ButtonWidget::initialize_state(true));
        game_mode_btns.insert(GameMode::Custom, ButtonWidget::initialize_state(true));
        let reset_btn = ButtonWidget::initialize_state(true);
        let next_btn = ButtonWidget::initialize_state(false);
        let rules_btn = ButtonWidget::initialize_state(true);
        let generate_btn = ButtonWidget::initialize_state(false);
        let id_text_btn = ButtonWidget::initialize_state(true);
        let heading = LabelWidget::initialize_state(constants::HEADING_TEXT);
        let pieces_label = LabelWidget::initialize_state(constants::PIECES_LABEL_TEXT);
        let max_age_label = LabelWidget::initialize_state(constants::AGE_LABEL_TEXT);
        let show_rules = false;
        let pieces_counter = CounterWidget::initialize_state(2, 7, settings.num_pieces, false);
        let moves_counter = CounterWidget::initialize_state(1, 7,  settings.max_moves_per_piece, false);
        game_mode_btns.get_mut(&game_mode).unwrap().is_active = false;

        Self {
            puzzle,
            board,
            resources,
            settings,
            game_mode,
            game_mode_btns,
            rules_btn_text: constants::RULES_BUTTON_TEXT.to_string(),
            reset_btn,
            next_btn,
            rules_btn,
            id_text_btn,
            show_rules,
            heading_text: constants::HEADING_TEXT.to_string(),
            heading,
            pieces_counter,
            max_age_counter: moves_counter,
            generate_btn,
            pieces_label,
            max_age_label,
            ..Default::default()
        }
    }

    pub fn handle_input(&mut self) {
        let (mx, my) = mouse_position();
        let winput = WidgetInput {
            mouse_pos: Circle::new(mx, my, 0.0),
        };

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

        self.id_text_btn.handle_input(
            &self.resources.sound(&SoundKind::Click),
            self.settings.volume,
        );

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
        let mut game_mode_changed = false;
        for (mode, btn) in &mut self.game_mode_btns {
            if btn
                .handle_input(
                    &self.resources.sound(&SoundKind::Mode),
                    self.settings.volume,
                )
                .is_clicked
            {
                self.game_mode = *mode;
                game_mode_changed = true;
                break;
            }
        }

        if game_mode_changed {
            for (mode, btn) in &mut self.game_mode_btns {
                if *mode == self.game_mode {
                    btn.is_active = false;
                } else {
                    btn.is_active = true;
                }
            }

            if self.game_mode != GameMode::Custom {
                self.generate_btn.is_active = false;
                self.pieces_counter.set_active(false);
                self.max_age_counter.set_active(false);
                self.next_puzzle();
            } else {
                self.generate_btn.is_active = true;
                self.pieces_counter.set_active(true);
                self.max_age_counter.set_active(true);
            }

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
            self.rules_btn_text = match self.show_rules {
                true => constants::RULES_BUTTON_ALT_TEXT.to_string(),
                false => constants::RULES_BUTTON_TEXT.to_string(),
            };

            return;
        }

        let pieces_counter_interaction =
            self.pieces_counter
                .handle_input(&winput, &self.resources, &self.settings);
        if pieces_counter_interaction.interacted {
            self.settings.num_pieces = pieces_counter_interaction.current;
            return;
        }

        let moves_counter_interaction =
            self.max_age_counter
                .handle_input(&winput, &self.resources, &self.settings);
        if moves_counter_interaction.interacted {
            self.settings.max_moves_per_piece = moves_counter_interaction.current;
            return;
        }

        if self
            .generate_btn
            .handle_input(
                &self.resources.sound(&SoundKind::Button),
                self.settings.volume,
            )
            .is_clicked
        {
            if self.game_mode == GameMode::Custom {
                self.next_puzzle();
            }
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
                self.rules_btn_text = constants::RULES_BUTTON_TEXT.to_string();
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
            self.puzzle = Game::generate_puzzle(self.game_mode, &self.settings);
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

    fn generate_puzzle(mode: GameMode, settings: &GameSettings) -> Puzzle {
        let piece_count = match mode {
            GameMode::Easy => 3,
            GameMode::Medium => 5,
            GameMode::Hard => 7,
            GameMode::Custom => settings.num_pieces,
        };

        let max_moves_per_piece = match mode {
            GameMode::Easy => 2,
            GameMode::Medium => 2,
            GameMode::Hard => 2,
            GameMode::Custom => settings.max_moves_per_piece,
        };

        let generated = generator::generate_weighted_random(
            piece_count,
            100,
            max_moves_per_piece,
            &MacroquadRandAdapter,
        );
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
