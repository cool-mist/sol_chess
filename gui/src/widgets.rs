use std::collections::{HashMap, HashSet};

mod board;
mod button;
mod controls;
mod counter;
mod label;

pub use board::*;
pub use button::*;
pub use controls::*;
pub use counter::*;
pub use label::*;

use macroquad::{
    audio::{self, Sound},
    prelude::*,
};
use quad_snd::PlaySoundParams;

use crate::{
    game::{Command, Settings},
    resources::Resources,
};

pub struct Interaction {
    pub mouse_pos: Circle,
    pub keys_released: HashSet<KeyCode>,
    pub mouse_pressed: HashSet<MouseButton>,
    pub mouse_released: HashSet<MouseButton>,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Default)]
pub struct WidgetId(pub u64);

pub enum Widget {
    Label(LabelWidget),
    Counter(CounterWidget),
    Button(ButtonWidget),
    Board(BoardWidget),
    Controls(ControlsWidget),
}

impl Widget {
    pub fn draw(&mut self, resources: &Resources, settings: &Settings) {
        match self {
            Widget::Label(label) => label.draw(&resources),
            Widget::Counter(counter) => counter.draw(&resources),
            Widget::Button(btn) => btn.draw(&resources),
            Widget::Board(board) => board.draw(&resources, &settings),
            Widget::Controls(_) => (),
        }
    }

    pub fn update(
        &mut self,
        interaction: &Interaction,
        resources: &Resources,
        settings: &Settings,
    ) -> Option<Command> {
        match self {
            Widget::Label(_) => None,
            Widget::Counter(c) => handle_counter_input(interaction, c, resources, settings),
            Widget::Button(b) => handle_button_input(interaction, b, resources, settings),
            Widget::Board(b) => handle_board_input(interaction, b, resources, settings),
            Widget::Controls(c) => c.handle_input(interaction),
        }
    }
}

fn handle_board_input(
    interaction: &Interaction,
    b: &mut BoardWidget,
    resources: &Resources,
    settings: &Settings,
) -> Option<Command> {
    let board_interaction = b.handle_input(interaction, resources, settings);
    let Some(board_interaction) = board_interaction else {
        return None;
    };

    Some((b.on_board_state_changed)(board_interaction))
}

fn handle_counter_input(
    interaction: &Interaction,
    counter: &mut CounterWidget,
    resources: &Resources,
    settings: &Settings,
) -> Option<Command> {
    let counter_interaction = counter.handle_input(interaction, resources, settings);
    let Some(interaction_type) = counter_interaction.interaction_type else {
        return None;
    };

    let command = match interaction_type {
        CounterInteractionType::Increment => (counter.on_increment)(counter_interaction),
        CounterInteractionType::Decrement => (counter.on_decrement)(counter_interaction),
    };

    Some(command)
}

fn handle_button_input(
    interaction: &Interaction,
    button: &mut ButtonWidget,
    resources: &Resources,
    settings: &Settings,
) -> Option<Command> {
    let Some(ButtonInteraction { clicked: true }) =
        button.handle_input(interaction, resources, settings)
    else {
        return None;
    };

    return Some((button.on_click)(button));
}

macro_rules! get_widget_fn {
    ($name:ident, $ret:ty, $variant:path) => {
        pub fn $name(widgets: &mut HashMap<WidgetId, Widget>, id: WidgetId) -> &mut $ret {
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

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiColor {
    Grey,
    Green,
    Pink,
    Brown,
    Yellow,
    Blue,
}

impl UiColor {
    pub fn to_bg_color(&self) -> Color {
        match self {
            UiColor::Grey => Color::from_rgba(140, 140, 140, 200),
            UiColor::Green => Color::from_rgba(16, 60, 50, 200),
            UiColor::Pink => Color::from_rgba(255, 60, 60, 200),
            UiColor::Brown => Color::from_rgba(123, 61, 35, 200),
            UiColor::Yellow => Color::from_rgba(242, 230, 190, 200),
            UiColor::Blue => Color::from_rgba(47, 85, 172, 200),
        }
    }

    pub fn to_fg_color(&self) -> Color {
        match self {
            UiColor::Grey => Color::from_rgba(255, 255, 255, 200),
            UiColor::Green => Color::from_rgba(255, 255, 255, 200),
            UiColor::Pink => Color::from_rgba(255, 255, 255, 200),
            UiColor::Brown => Color::from_rgba(255, 255, 255, 200),
            UiColor::Yellow => Color::from_rgba(0, 0, 0, 200),
            UiColor::Blue => Color::from_rgba(255, 255, 255, 200),
        }
    }

    pub fn to_shadow_color(&self) -> Color {
        let bg_color = self.to_bg_color();
        Color::from_rgba(
            (bg_color.r * 255.) as u8,
            (bg_color.g * 255.) as u8,
            (bg_color.b * 255.) as u8,
            100,
        )
    }
}

pub fn draw_shadow(rect: &Rect, shadow_width: f32) {
    let shadow_color = Color::new(0., 0., 0., 0.8);
    draw_rectangle(
        rect.x + rect.w,
        rect.y + shadow_width,
        shadow_width,
        rect.h,
        shadow_color,
    );

    draw_rectangle(
        rect.x + shadow_width,
        rect.y + rect.h,
        rect.w - shadow_width,
        shadow_width,
        shadow_color,
    );
}

pub fn play_sound_once(sound: &Sound, volume: f32) {
    audio::play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume,
        },
    );
}
