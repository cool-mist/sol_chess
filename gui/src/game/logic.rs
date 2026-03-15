use std::collections::HashMap;

use super::{Game, GameMode, Settings, constants};
use crate::{
    game::{self, Command},
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
        let settings = Settings {
            volume: constants::VOLUME,
            max_age: 3,
            max_pieces: 2,
            ..Default::default()
        };
        let puzzle = generate_new_puzzle(game_mode, &settings);
        let game_mode = GameMode::Medium;
        let show_rules = false;

        let mut widgets = HashMap::new();

        let w_label_heading = LabelWidget::new(constants::HEADING_TEXT);
        let label_heading = add_widget(&mut widgets, Widget::Label(w_label_heading));

        let w_label_pieces = LabelWidget::new(constants::PIECES_LABEL_TEXT);
        let label_pieces = add_widget(&mut widgets, Widget::Label(w_label_pieces));

        let w_label_max_age = LabelWidget::new(constants::AGE_LABEL_TEXT);
        let label_max_age = add_widget(&mut widgets, Widget::Label(w_label_max_age));

        let mut w_counter_pieces =
            CounterWidget::new(2, 7, settings.max_pieces, false, UiColor::Yellow);
        w_counter_pieces.on_increment = |c| Command::UpdateMaxPieceCount(c.current);
        w_counter_pieces.on_decrement = |c| Command::UpdateMaxPieceCount(c.current);
        let counter_pieces = add_widget(&mut widgets, Widget::Counter(w_counter_pieces));

        let mut w_counter_moves =
            CounterWidget::new(1, 7, settings.max_age, false, UiColor::Yellow);
        w_counter_moves.on_increment = |c| Command::UpdateMaxAge(c.current);
        w_counter_moves.on_decrement = |c| Command::UpdateMaxAge(c.current);
        let counter_moves = add_widget(&mut widgets, Widget::Counter(w_counter_moves));

        let mut w_btn_reset = ButtonWidget::new(
            true,
            UiColor::Yellow,
            constants::RESET_BUTTON_TEXT,
            SoundKind::Button,
        );
        w_btn_reset.on_click = |_| Command::ResetBoard;
        let btn_reset = add_widget(&mut widgets, Widget::Button(w_btn_reset));

        let mut w_btn_next = ButtonWidget::new(
            false,
            UiColor::Green,
            constants::NEXT_BUTTON_TEXT,
            SoundKind::Button,
        );
        w_btn_next.on_click = |_| Command::NextPuzzle;
        let btn_next = add_widget(&mut widgets, Widget::Button(w_btn_next));

        let mut w_btn_rules = ButtonWidget::new(
            true,
            UiColor::Brown,
            constants::RULES_BUTTON_TEXT,
            SoundKind::Button,
        );
        w_btn_rules.on_click = |_| Command::ToggleRules;
        let btn_rules = add_widget(&mut widgets, Widget::Button(w_btn_rules));

        let mut w_btn_generate = ButtonWidget::new(
            false,
            UiColor::Pink,
            constants::GENERATE_BUTTON_TEXT,
            SoundKind::Button,
        );
        w_btn_generate.on_click = |_| Command::NewCustomPuzzle;
        let btn_generate = add_widget(&mut widgets, Widget::Button(w_btn_generate));

        let mut game_mode_btns = HashMap::new();

        let mut w_game_mode_easy_btn = ButtonWidget::new(
            true,
            UiColor::Yellow,
            constants::EASY_BUTTON_TEXT,
            SoundKind::Mode,
        );
        w_game_mode_easy_btn.on_click = |_| Command::SwitchGameMode(GameMode::Easy);
        game_mode_btns.insert(
            GameMode::Easy,
            add_widget(&mut widgets, Widget::Button(w_game_mode_easy_btn)),
        );

        let mut w_game_mode_medium_btn = ButtonWidget::new(
            true,
            UiColor::Yellow,
            constants::MEDIUM_BUTTON_TEXT,
            SoundKind::Mode,
        );
        w_game_mode_medium_btn.on_click = |_| Command::SwitchGameMode(GameMode::Medium);
        game_mode_btns.insert(
            GameMode::Medium,
            add_widget(&mut widgets, Widget::Button(w_game_mode_medium_btn)),
        );

        let mut w_game_mode_hard_btn = ButtonWidget::new(
            true,
            UiColor::Yellow,
            constants::HARD_BUTTON_TEXT,
            SoundKind::Mode,
        );
        w_game_mode_hard_btn.on_click = |_| Command::SwitchGameMode(GameMode::Hard);
        game_mode_btns.insert(
            GameMode::Hard,
            add_widget(&mut widgets, Widget::Button(w_game_mode_hard_btn)),
        );

        let mut w_game_mode_custom_btn = ButtonWidget::new(
            true,
            UiColor::Blue,
            constants::CUSTOM_BUTTON_TEXT,
            SoundKind::Mode,
        );
        w_game_mode_custom_btn.on_click = |_| Command::SwitchGameMode(GameMode::Custom);
        game_mode_btns.insert(
            GameMode::Custom,
            add_widget(&mut widgets, Widget::Button(w_game_mode_custom_btn)),
        );

        let active_game_mode_btn = game_mode_btns.get(&game_mode).unwrap();
        game::button(&mut widgets, *active_game_mode_btn).is_active = false;

        let mut w_board = BoardWidget::new(puzzle.board.size, puzzle.board.clone());
        w_board.on_board_state_changed = |s| Command::UpdateBoard(s.new_board_state);
        let board = add_widget(&mut widgets, Widget::Board(w_board));

        let mut w_controls = ControlsWidget::new();
        w_controls.add(KeyCode::Q, Command::Quit);
        w_controls.add(KeyCode::Escape, Command::HideRules);
        w_controls.add(KeyCode::D, Command::ToggleDebug);
        add_widget(&mut widgets, Widget::Controls(w_controls));

        Game {
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
            btn_generate,
            label_pieces,
            label_max_age,
            label_heading,
            ..Default::default()
        }
    }

    pub fn update(&mut self, interaction: Interaction) {
        let command = self.map_to_command(interaction);
        self.update_state(command);
    }

    fn map_to_command(&mut self, interaction: Interaction) -> Command {
        for (_, widget) in &mut self.widgets {
            if let Some(command) = widget.update(&interaction, &self.resources, &self.settings) {
                return command;
            }
        }

        return Command::NoInteraction;
    }

    fn update_state(&mut self, command: Command) {
        match command {
            Command::NoInteraction => (),
            Command::UpdateBoard(new_board_state) => self.update_board(new_board_state),
            Command::SwitchGameMode(mode) => self.switch_game_mode(mode),
            Command::ToggleDebug => self.settings.debug = !self.settings.debug,
            Command::HideRules => self.hide_rules(),
            Command::ResetBoard => self.reset(),
            Command::NextPuzzle => self.next_puzzle(),
            Command::ToggleRules => self.toggle_rules(),
            Command::UpdateMaxPieceCount(new_value) => self.settings.max_pieces = new_value,
            Command::UpdateMaxAge(new_value) => self.settings.max_age = new_value,
            Command::NewCustomPuzzle => self.new_custom_puzzle(),
            Command::Quit => std::process::exit(0),
        }
    }

    fn new_custom_puzzle(&mut self) {
        if self.game_mode == GameMode::Custom {
            self.next_puzzle();
        }
    }

    fn toggle_rules(&mut self) {
        self.show_rules = !self.show_rules;
        w_board!(self, board).show_rules = self.show_rules;
        w_button!(self, btn_rules).text = match self.show_rules {
            true => constants::RULES_BUTTON_ALT_TEXT.to_string(),
            false => constants::RULES_BUTTON_TEXT.to_string(),
        };
    }

    fn hide_rules(&mut self) {
        if self.show_rules {
            play_sound_once(
                &self.resources.sound(&SoundKind::Button),
                self.settings.volume,
            );
            self.show_rules = false;
            w_board!(self, board).show_rules = false;
            w_button!(self, btn_rules).text = constants::RULES_BUTTON_TEXT.to_string();
        }
    }

    fn switch_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
        for (mode, btn_id) in &mut self.game_mode_btns {
            // The active mode button is 'disabled' to indicate it is active
            let is_active = !(*mode == self.game_mode);
            game::button(&mut self.widgets, *btn_id).is_active = is_active;
        }

        let is_custom_game_mode = self.game_mode == GameMode::Custom;
        w_button!(self, btn_generate).is_active = is_custom_game_mode;
        w_counter!(self, counter_pieces).set_active(is_custom_game_mode);
        w_counter!(self, counter_moves).set_active(is_custom_game_mode);
        self.next_puzzle();
    }

    fn update_board(&mut self, new_board_state: BoardState) {
        match new_board_state {
            BoardState::Won => {
                w_button!(self, btn_next).is_active = true;
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.reset_game(Default::default());
    }

    fn reset_game(&mut self, options: ResetOptions) {
        if options.create_new_puzzle {
            self.puzzle = generate_new_puzzle(self.game_mode, &self.settings);
        }

        w_button!(self, btn_next).is_active = false;
        w_board!(self, board).reset(&self.puzzle);
    }

    fn next_puzzle(&mut self) {
        self.reset_game(ResetOptions {
            create_new_puzzle: true,
            ..Default::default()
        });
    }
}

fn add_widget(widgets: &mut HashMap<WidgetId, Widget>, w: Widget) -> WidgetId {
    let id = rand::rand() as u64;
    let widget_id = WidgetId(id);
    widgets.insert(widget_id, w);

    widget_id
}

fn generate_new_puzzle(mode: GameMode, settings: &Settings) -> Puzzle {
    let piece_count = match mode {
        GameMode::Easy => 3,
        GameMode::Medium => 5,
        GameMode::Hard => 7,
        GameMode::Custom => settings.max_pieces,
    };

    let max_moves_per_piece = match mode {
        GameMode::Easy => 2,
        GameMode::Medium => 2,
        GameMode::Hard => 2,
        GameMode::Custom => settings.max_age,
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

#[derive(Default)]
struct ResetOptions {
    create_new_puzzle: bool,
}

pub struct MacroquadRandAdapter;
impl RandomRange for MacroquadRandAdapter {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        rand::gen_range(min, max)
    }
}
