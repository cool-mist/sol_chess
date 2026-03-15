pub mod constants;
mod draw;
mod logic;

use std::collections::HashMap;

use crate::{
    game,
    resources::{self, Resources, SoundKind},
    widgets::*,
};
use macroquad::prelude::*;
use sol_lib::{board::BoardState, generator::Puzzle};

#[derive(Default)]
pub struct Game {
    resources: Resources,

    // Game state
    settings: Settings,
    puzzle: Puzzle,
    show_rules: bool,
    game_mode: GameMode,
    game_mode_btns: HashMap<GameMode, WidgetId>,

    // Render state
    window_height: f32,
    window_width: f32,

    widgets: HashMap<WidgetId, Widget>,

    board: WidgetId,

    counter_pieces: WidgetId,
    counter_moves: WidgetId,

    label_pieces: WidgetId,
    label_max_age: WidgetId,
    label_heading: WidgetId,

    btn_rules: WidgetId,
    btn_reset: WidgetId,
    btn_next: WidgetId,
    btn_generate: WidgetId,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Default)]
pub struct WidgetId(u64);

pub enum Widget {
    Label(LabelWidget),
    Counter(CounterWidget),
    Button(ButtonWidget),
    Board(BoardWidget),
}

impl Widget {
    fn draw(&mut self, resources: &Resources, settings: &Settings) {
        match self {
            Widget::Label(label) => label.draw(&resources),
            Widget::Counter(counter) => counter.draw(&resources),
            Widget::Button(btn) => btn.draw(&resources),
            Widget::Board(board) => board.draw(&resources, &settings),
        }
    }

    fn update(
        &mut self,
        interaction: &Interaction,
        resources: &Resources,
        settings: &Settings,
    ) -> Command {
        match self {
            Widget::Label(_) => Command::NoInteraction,
            Widget::Counter(c) => handle_counter_input(interaction, c, resources, settings),
            Widget::Button(b) => handle_button_input(interaction, b, resources, settings),
            Widget::Board(_) => Command::NoInteraction,
        }
    }
}

fn handle_counter_input(
    interaction: &Interaction,
    counter: &mut CounterWidget,
    resources: &Resources,
    settings: &Settings,
) -> Command {
    let counter_interaction = counter.handle_input(interaction, resources, settings);
    let Some(interaction_type) = counter_interaction.interaction_type else {
        return Command::NoInteraction;
    };

    match interaction_type {
        CounterInteractionType::Increment => (counter.on_increment)(counter_interaction),
        CounterInteractionType::Decrement => (counter.on_decrement)(counter_interaction),
    }
}

fn handle_button_input(
    interaction: &Interaction,
    button: &mut ButtonWidget,
    resources: &Resources,
    settings: &Settings,
) -> Command {
    let clicked = button
        .handle_input(interaction, resources, settings)
        .clicked;
    if clicked {
        return (button.on_click)(button);
    }

    Command::NoInteraction
}

macro_rules! get_widget_fn {
    ($name:ident, $ret:ty, $variant:path) => {
        fn $name(widgets: &mut HashMap<WidgetId, Widget>, id: WidgetId) -> &mut $ret {
            match widgets.get_mut(&id) {
                Some(w) => match w {
                    $variant(v) => return v,
                    _ => panic!("Widget with id {} not found", id.0),
                },
                None => panic!("Widget with id {} not found", id.0),
            }
        }
    };
}

get_widget_fn!(label, LabelWidget, Widget::Label);
get_widget_fn!(button, ButtonWidget, Widget::Button);
get_widget_fn!(counter, CounterWidget, Widget::Counter);
get_widget_fn!(board, BoardWidget, Widget::Board);

macro_rules! widget_accessor {
    ($name:ident, $fn:ident) => {
        macro_rules! $name {
            ($this:expr, $widget_id:ident) => {
                game::$fn(&mut $this.widgets, $this.$widget_id)
            };
        }
        pub(crate) use $name;
    };
}

widget_accessor!(w_button, button);
widget_accessor!(w_label, label);
widget_accessor!(w_counter, counter);
widget_accessor!(w_board, board);

#[derive(Default)]
pub struct Settings {
    pub volume: f32,
    pub debug: bool,
    pub max_pieces: u32,
    pub max_age: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameMode {
    Easy,
    Medium,
    Hard,
    Custom,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Medium
    }
}

pub enum Command {
    NoInteraction,
    ResetBoard,
    NextPuzzle,
    ToggleRules,
    UpdateBoard(BoardState),
    SwitchGameMode(GameMode),
    UpdateMaxPieceCount(u32),
    UpdateMaxAge(u32),
    NewCustomPuzzle,
    ToggleDebug,
    HideRules,
    Quit,
}

pub async fn new_game() -> Game {
    let resources = resources::init().await;
    let game_mode = GameMode::Medium;
    let settings = Settings {
        volume: constants::VOLUME,
        max_age: 3,
        max_pieces: 2,
        ..Default::default()
    };
    let puzzle = logic::generate_new_puzzle(game_mode, &settings);
    let game_mode = GameMode::Medium;
    let label_heading = LabelWidget::initialize_state(constants::HEADING_TEXT);
    let label_pieces = LabelWidget::initialize_state(constants::PIECES_LABEL_TEXT);
    let label_max_age = LabelWidget::initialize_state(constants::AGE_LABEL_TEXT);
    let show_rules = false;

    let mut widgets = HashMap::new();
    let label_pieces = add_widget(&mut widgets, Widget::Label(label_pieces));
    let label_max_age = add_widget(&mut widgets, Widget::Label(label_max_age));
    let label_heading = add_widget(&mut widgets, Widget::Label(label_heading));

    let mut w_counter_pieces =
        CounterWidget::new(2, 7, settings.max_pieces, false, UiColor::Yellow);
    w_counter_pieces.on_increment = |c| Command::UpdateMaxPieceCount(c.current);
    w_counter_pieces.on_decrement = |c| Command::UpdateMaxPieceCount(c.current);
    let counter_pieces = add_widget(&mut widgets, Widget::Counter(w_counter_pieces));

    let mut w_counter_moves = CounterWidget::new(1, 7, settings.max_age, false, UiColor::Yellow);
    w_counter_moves.on_increment = |c| Command::UpdateMaxAge(c.current);
    w_counter_moves.on_decrement = |c| Command::UpdateMaxAge(c.current);
    let counter_moves = add_widget(&mut widgets, Widget::Counter(w_counter_moves));

    let btn_reset = add_widget(
        &mut widgets,
        Widget::Button(ButtonWidget::new(
            true,
            UiColor::Yellow,
            constants::RESET_BUTTON_TEXT,
            SoundKind::Button,
        )),
    );

    let btn_next = add_widget(
        &mut widgets,
        Widget::Button(ButtonWidget::new(
            false,
            UiColor::Green,
            constants::NEXT_BUTTON_TEXT,
            SoundKind::Button,
        )),
    );

    let btn_rules = add_widget(
        &mut widgets,
        Widget::Button(ButtonWidget::new(
            true,
            UiColor::Brown,
            constants::RULES_BUTTON_TEXT,
            SoundKind::Button,
        )),
    );

    let btn_generate = add_widget(
        &mut widgets,
        Widget::Button(ButtonWidget::new(
            false,
            UiColor::Pink,
            constants::GENERATE_BUTTON_TEXT,
            SoundKind::Button,
        )),
    );

    let mut game_mode_btns = HashMap::new();
    game_mode_btns.insert(
        GameMode::Easy,
        add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                true,
                UiColor::Yellow,
                constants::EASY_BUTTON_TEXT,
                SoundKind::Mode,
            )),
        ),
    );

    game_mode_btns.insert(
        GameMode::Medium,
        add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                true,
                UiColor::Yellow,
                constants::MEDIUM_BUTTON_TEXT,
                SoundKind::Mode,
            )),
        ),
    );

    game_mode_btns.insert(
        GameMode::Hard,
        add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                true,
                UiColor::Yellow,
                constants::HARD_BUTTON_TEXT,
                SoundKind::Mode,
            )),
        ),
    );

    game_mode_btns.insert(
        GameMode::Custom,
        add_widget(
            &mut widgets,
            Widget::Button(ButtonWidget::new(
                true,
                UiColor::Blue,
                constants::CUSTOM_BUTTON_TEXT,
                SoundKind::Mode,
            )),
        ),
    );

    let active_game_mode_btn = game_mode_btns.get(&game_mode).unwrap();
    game::button(&mut widgets, *active_game_mode_btn).is_active = false;

    let board = add_widget(
        &mut widgets,
        Widget::Board(BoardWidget::initialize_state(
            puzzle.board.size,
            puzzle.board.clone(),
        )),
    );

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

fn add_widget(widgets: &mut HashMap<WidgetId, Widget>, w: Widget) -> WidgetId {
    let id = rand::rand() as u64;
    let widget_id = WidgetId(id);
    widgets.insert(widget_id, w);

    widget_id
}
