use super::{Game, GameMode, Settings, constants};
use crate::{
    game::{self, Command, w_board, w_button, w_counter},
    resources::*,
    widgets::*,
};
use macroquad::prelude::*;
use sol_lib::{
    board::BoardState,
    generator::{self, Puzzle, RandomRange},
};

impl Game {
    pub fn update(&mut self, interaction: Interaction) {
        let command = self.get_command(interaction);
        self.update_state(command);
    }

    fn get_command(&mut self, interaction: Interaction) -> Command {

        for (_, widget) in &mut self.widgets {
            let cmd = widget.update(&interaction, &self.resources, &self.settings);
            let ret_cmd = match cmd {
                Command::NoInteraction => false,
                _ => true,
            };

            if ret_cmd {
                return cmd;
            }
        }

        let board_interaction =
            w_board!(self, board).handle_input(&interaction, &self.resources, &self.settings);
        if let Some(new_board_state) = board_interaction.new_board_state {
            return Command::UpdateBoard(new_board_state);
        }

        if w_button!(self, btn_reset)
            .handle_input(&interaction, &self.resources, &self.settings)
            .clicked
        {
            return Command::ResetBoard;
        }

        if w_button!(self, btn_next)
            .handle_input(&interaction, &self.resources, &self.settings)
            .clicked
        {
            return Command::NextPuzzle;
        }

        for (mode, btn) in &mut self.game_mode_btns {
            if game::button(&mut self.widgets, *btn)
                .handle_input(&interaction, &self.resources, &self.settings)
                .clicked
            {
                return Command::SwitchGameMode(*mode);
            }
        }

        if w_button!(self, btn_rules)
            .handle_input(&interaction, &self.resources, &self.settings)
            .clicked
        {
            return Command::ToggleRules;
        }

        if w_button!(self, btn_generate)
            .handle_input(&interaction, &self.resources, &self.settings)
            .clicked
        {
            return Command::NewCustomPuzzle;
        }

        if interaction.keys_released.contains(&KeyCode::Escape) {
            return Command::HideRules;
        }

        if interaction.keys_released.contains(&KeyCode::D) {
            return Command::ToggleDebug;
        }

        if interaction.keys_released.contains(&KeyCode::Q) {
            return Command::Quit;
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

pub fn generate_new_puzzle(mode: GameMode, settings: &Settings) -> Puzzle {
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
