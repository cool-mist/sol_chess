pub mod constants;
mod draw;
mod logic;

use std::collections::HashMap;

use crate::{resources::Resources, widgets::*};
use macroquad::prelude::*;
use sol_lib::generator::Puzzle;

#[derive(Default)]
pub struct Game {
    // Game state
    settings: GameSettings,
    puzzle: Puzzle,
    show_rules: bool,
    game_mode: GameMode,

    // Render state
    resources: Resources,

    window_height: f32,
    window_width: f32,

    rules_font_size: f32, // TODO: remove

    generate_btn: ButtonWidget,
    board: BoardWidget,
    game_mode_btns: HashMap<GameMode, ButtonWidget>,

    widgets: HashMap<WidgetId, Widget>,

    counter_pieces: WidgetId,
    counter_moves: WidgetId,

    label_pieces: WidgetId,
    label_max_age: WidgetId,
    label_heading: WidgetId,

    btn_rules: WidgetId,
    btn_reset: WidgetId,
    btn_next: WidgetId,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Default)]
pub struct WidgetId(u64);

pub enum Widget {
    Label(LabelWidget),
    Counter(CounterWidget),
    Button(ButtonWidget),
}

impl Widget {
    fn draw(&mut self, resources: &Resources) {
        match self {
            Widget::Label(label) => label.draw(&resources),
            Widget::Counter(counter) => counter.draw(&resources),
            Widget::Button(btn) => btn.draw(&resources),
        }
    }
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

#[derive(Default)]
pub struct GameSettings {
    pub volume: f32,
    pub max_moves_per_piece: u32,
    pub debug: bool,
    pub num_pieces: u32,
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
