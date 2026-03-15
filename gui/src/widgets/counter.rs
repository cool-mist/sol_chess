use crate::{
    game::{Command, Settings},
    resources::{Resources, SoundKind},
    widgets::*,
};
use macroquad::prelude::*;

pub struct CounterWidget {
    visual_rect: Rect,
    increment: ButtonWidget,
    decrement: ButtonWidget,
    pub on_increment: fn(CounterWidgetInteraction) -> Command,
    pub on_decrement: fn(CounterWidgetInteraction) -> Command,
    min: u32,
    max: u32,
    current: u32,
    font_size: f32,
    visual_dims: TextDimensions,
    increment_dims: TextDimensions,
    decrement_dims: TextDimensions,
    is_active: bool,
}

pub struct CounterWidgetInteraction {
    pub current: u32,
    pub interaction_type: Option<CounterInteractionType>,
}

#[derive(Copy, Clone)]
pub enum CounterInteractionType {
    Increment,
    Decrement,
}

impl CounterWidgetInteraction {
    fn decrement(current: u32) -> Self {
        Self {
            current,
            interaction_type: Some(CounterInteractionType::Decrement),
        }
    }

    fn increment(current: u32) -> Self {
        Self {
            current,
            interaction_type: Some(CounterInteractionType::Increment),
        }
    }

    fn not_interacted(current: u32) -> Self {
        Self {
            current,
            interaction_type: None,
        }
    }
}

impl CounterWidget {
    pub fn new(min: u32, max: u32, current: u32, is_active: bool, color: UiColor) -> Self {
        let increment = ButtonWidget::new(is_active, color, "+", SoundKind::Click);
        let decrement = ButtonWidget::new(is_active, color, "-", SoundKind::Click);

        Self {
            min,
            max,
            current,
            is_active,
            increment,
            decrement,
            ..Default::default()
        }
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
        self.increment.is_active = is_active;
        self.decrement.is_active = is_active;
    }

    pub fn place(&mut self, total_area: Rect, font_size: f32, resources: &Resources) {
        let margin_y = 0.05 * total_area.h;
        let btn_h = total_area.h * 0.3;
        let btn_w = total_area.w * 0.9;
        let btn_x = total_area.x + (total_area.w - btn_w) / 2.;
        let increment_y = total_area.y + margin_y;
        let visual_y = increment_y + btn_h + margin_y;
        let decrement_y = visual_y + btn_h + margin_y;

        self.visual_rect = Rect::new(btn_x, visual_y, btn_w, btn_h);

        let increment_rect = Rect::new(btn_x, increment_y, btn_w, btn_h);
        self.increment.place(increment_rect, FontSize::Fill);

        let decrement_rect = Rect::new(btn_x, decrement_y, btn_w, btn_h);
        self.decrement.place(decrement_rect, FontSize::Fill);

        self.increment_dims = measure_text("+", Some(resources.font()), font_size as u16, 1.0);
        self.decrement_dims = measure_text("-", Some(resources.font()), font_size as u16, 1.0);

        self.visual_dims = measure_text("0", Some(resources.font()), font_size as u16, 1.0);
        self.font_size = font_size;
    }

    pub fn handle_input(
        &mut self,
        interaction: &Interaction,
        resources: &Resources,
        settings: &Settings,
    ) -> CounterWidgetInteraction {
        if let Some(interaction) = self
            .increment
            .handle_input(&interaction, &resources, &settings)
        {
            if interaction.clicked {
                if self.current < self.max {
                    self.current += 1;
                    return CounterWidgetInteraction::increment(self.current);
                }
            }
        }

        if let Some(interaction) = self
            .decrement
            .handle_input(&interaction, &resources, &settings)
        {
            if interaction.clicked {
                if self.current > self.min {
                    self.current -= 1;
                    return CounterWidgetInteraction::decrement(self.current);
                }
            }
        }

        return CounterWidgetInteraction::not_interacted(self.current);
    }

    pub fn draw(&mut self, resources: &Resources) {
        self.increment.draw(resources);
        self.decrement.draw(resources);

        let font_size = self.font_size as u16;
        draw_text_ex(
            &self.current.to_string(),
            self.visual_rect.x + (self.visual_rect.w - self.visual_dims.width) / 2.,
            self.visual_rect.y + (self.visual_rect.h + self.font_size) / 2.,
            TextParams {
                font: Some(resources.font()),
                font_size,
                color: UiColor::Yellow.to_fg_color(),
                ..Default::default()
            },
        );
    }
}

impl Default for CounterWidget {
    fn default() -> Self {
        Self {
            on_increment: |_| Command::NoInteraction,
            on_decrement: |_| Command::NoInteraction,
            visual_rect: Default::default(),
            increment: Default::default(),
            decrement: Default::default(),
            min: Default::default(),
            max: Default::default(),
            current: Default::default(),
            font_size: Default::default(),
            visual_dims: Default::default(),
            increment_dims: Default::default(),
            decrement_dims: Default::default(),
            is_active: Default::default(),
        }
    }
}
