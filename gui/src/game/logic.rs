use std::collections::HashMap;

use super::{Game, GameMode, GameSettings, constants};
use crate::{
    game::{self, Widget, WidgetId, w_button, w_counter},
    resources::{self, *},
    widgets::*,
};
use macroquad::prelude::*;
use sol_lib::{
    board::BoardState,
    generator::{self, Puzzle, RandomRange},
};

impl Game {
    pub async fn new() -> Self {
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
        game_mode_btns.insert(
            GameMode::Easy,
            ButtonWidget::new(true, UiColor::Yellow, constants::EASY_BUTTON_TEXT),
        );
        game_mode_btns.insert(
            GameMode::Medium,
            ButtonWidget::new(true, UiColor::Yellow, constants::MEDIUM_BUTTON_TEXT),
        );
        game_mode_btns.insert(
            GameMode::Hard,
            ButtonWidget::new(true, UiColor::Yellow, constants::HARD_BUTTON_TEXT),
        );
        game_mode_btns.insert(
            GameMode::Custom,
            ButtonWidget::new(true, UiColor::Blue, constants::CUSTOM_BUTTON_TEXT),
        );
        let generate_btn = ButtonWidget::new(false, UiColor::Pink, constants::GENERATE_BUTTON_TEXT);
        let heading = LabelWidget::initialize_state(constants::HEADING_TEXT);
        let pieces_label = LabelWidget::initialize_state(constants::PIECES_LABEL_TEXT);
        let max_age_label = LabelWidget::initialize_state(constants::AGE_LABEL_TEXT);
        let show_rules = false;
        game_mode_btns.get_mut(&game_mode).unwrap().is_active = false;

        let mut widgets = HashMap::new();
        let label_pieces = add_widget(&mut widgets, Widget::Label(pieces_label));
        let label_max_age = add_widget(&mut widgets, Widget::Label(max_age_label));
        let label_heading = add_widget(&mut widgets, Widget::Label(heading));

        let counter_pieces = add_widget(
            &mut widgets,
            Widget::Counter(CounterWidget::new(
                2,
                7,
                settings.num_pieces,
                false,
                UiColor::Yellow,
            )),
        );

        let counter_moves = add_widget(
            &mut widgets,
            Widget::Counter(CounterWidget::new(
                1,
                7,
                settings.max_moves_per_piece,
                false,
                UiColor::Yellow,
            )),
        );

        let btn_reset = add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                true,
                UiColor::Yellow,
                constants::RESET_BUTTON_TEXT,
            )),
        );

        let btn_next = add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                false,
                UiColor::Green,
                constants::NEXT_BUTTON_TEXT,
            )),
        );

        let btn_rules = add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                true,
                UiColor::Brown,
                constants::RULES_BUTTON_TEXT,
            )),
        );

        Self {
            puzzle,
            board,
            resources,
            settings,
            game_mode,
            game_mode_btns,
            btn_reset,
            btn_next,
            btn_rules,
            show_rules,
            widgets,
            counter_pieces,
            counter_moves,
            generate_btn,
            label_pieces,
            label_max_age,
            label_heading,
            ..Default::default()
        }
    }

    pub fn handle_interaction(&mut self, interaction: GameInteraction) {
        match interaction {
            GameInteraction::NoInteraction => {}
            GameInteraction::Board(new_board_state) => match new_board_state {
                BoardState::Won => {
                    w_button!(self, btn_next).is_active = true;
                }
                _ => {}
            },
            GameInteraction::ButtonRelease(button_name) => match button_name {
                ButtonName::Reset => self.reset(),
                ButtonName::Next => self.next_puzzle(),
                ButtonName::Rules => {
                    self.show_rules = !self.show_rules;
                    w_button!(self, btn_rules).text = match self.show_rules {
                        true => constants::RULES_BUTTON_ALT_TEXT.to_string(),
                        false => constants::RULES_BUTTON_TEXT.to_string(),
                    };
                }
                ButtonName::Generate => {
                    if self.game_mode == GameMode::Custom {
                        self.next_puzzle();
                    }
                }
            },
            GameInteraction::GameMode(mode) => {
                self.game_mode = mode;
                for (mode, btn) in &mut self.game_mode_btns {
                    if *mode == self.game_mode {
                        btn.is_active = false;
                    } else {
                        btn.is_active = true;
                    }
                }

                let is_custom_game_mode = self.game_mode == GameMode::Custom;
                self.generate_btn.is_active = is_custom_game_mode;
                w_counter!(self, counter_pieces).set_active(is_custom_game_mode);
                w_counter!(self, counter_moves).set_active(is_custom_game_mode);

                // A change in game mode, immediately generate new puzzle
                if !is_custom_game_mode {
                    self.next_puzzle();
                }
            }
            GameInteraction::Counter(counter_name, new_value) => match counter_name {
                CounterName::Pieces => {
                    self.settings.num_pieces = new_value;
                }
                CounterName::Age => {
                    self.settings.max_moves_per_piece = new_value;
                }
            },
            GameInteraction::ToggleDebug => self.settings.debug = !self.settings.debug,
            GameInteraction::HideRules => {
                if self.show_rules {
                    play_sound_once(
                        &self.resources.sound(&SoundKind::Button),
                        self.settings.volume,
                    );
                    self.show_rules = false;
                    w_button!(self, btn_rules).text = constants::RULES_BUTTON_TEXT.to_string();
                }
            }
            GameInteraction::Quit => std::process::exit(0),
        }
    }

    pub fn handle_user_input(&mut self) -> GameInteraction {
        let (mx, my) = mouse_position();
        let winput = UserInteraction {
            mouse_pos: Circle::new(mx, my, 0.0),
        };

        // WIDGETS
        let board_interaction = self
            .board
            .handle_input(&winput, &self.resources, &self.settings);
        if let Some(new_board_state) = board_interaction.new_board_state {
            return GameInteraction::Board(new_board_state);
        }

        if w_button!(self, btn_reset).handle_input(
            &self.resources.sound(&SoundKind::Button),
            self.settings.volume,
        ) {
            return GameInteraction::ButtonRelease(ButtonName::Reset);
        }

        if w_button!(self, btn_next).handle_input(
            &self.resources.sound(&SoundKind::Button),
            self.settings.volume,
        ) {
            return GameInteraction::ButtonRelease(ButtonName::Next);
        }

        for (mode, btn) in &mut self.game_mode_btns {
            if btn.handle_input(
                &self.resources.sound(&SoundKind::Mode),
                self.settings.volume,
            ) {
                return GameInteraction::GameMode(*mode);
            }
        }

        if w_button!(self, btn_rules).handle_input(
            &self.resources.sound(&SoundKind::Button),
            self.settings.volume,
        ) {
            return GameInteraction::ButtonRelease(ButtonName::Rules);
        }

        let pieces_counter_interaction =
            w_counter!(self, counter_pieces).handle_input(&winput, &self.resources, &self.settings);
        if pieces_counter_interaction.interacted {
            return GameInteraction::Counter(
                CounterName::Pieces,
                pieces_counter_interaction.current,
            );
        }

        let age_counter_interaction =
            w_counter!(self, counter_moves).handle_input(&winput, &self.resources, &self.settings);
        if age_counter_interaction.interacted {
            return GameInteraction::Counter(CounterName::Age, age_counter_interaction.current);
        }

        if self.generate_btn.handle_input(
            &self.resources.sound(&SoundKind::Button),
            self.settings.volume,
        ) {
            return GameInteraction::ButtonRelease(ButtonName::Generate);
        }

        // KEY INPUTS
        if is_key_released(KeyCode::Escape) {
            return GameInteraction::HideRules;
        }

        if is_key_released(KeyCode::D) {
            return GameInteraction::ToggleDebug;
        }

        if is_key_released(KeyCode::Q) {
            return GameInteraction::Quit;
        }

        return GameInteraction::NoInteraction;
    }

    fn reset(&mut self) {
        self.reset_game(Default::default());
    }

    fn reset_game(&mut self, options: ResetOptions) {
        if options.create_new_puzzle {
            self.puzzle = Game::generate_puzzle(self.game_mode, &self.settings);
        }

        w_button!(self, btn_next).is_active = false;
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

fn add_widget(widgets: &mut HashMap<WidgetId, Widget>, w: Widget) -> WidgetId {
    let id = rand::rand() as u64;
    let widget_id = WidgetId(id);
    widgets.insert(widget_id, w);

    widget_id
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
