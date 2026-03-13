use crate::{
    game::GameSettings,
    resources::{Resources, SoundKind},
    widgets::*,
};
use macroquad::prelude::*;

#[derive(Default)]
pub struct CounterWidget {
    visual_rect: Rect,
    increment: ButtonWidget,
    decrement: ButtonWidget,
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
    pub interacted: bool,
}

impl CounterWidget {
    pub fn new(
        min: u32,
        max: u32,
        current: u32,
        is_active: bool,
        color: UiColor,
    ) -> Self {
        let increment = ButtonWidget::new(is_active, color, "+");
        let decrement = ButtonWidget::new(is_active, color, "-");
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

    pub fn initialize_drawables(
        &mut self,
        total_area: Rect,
        font_size: f32,
        resources: &Resources,
    ) {
        let margin_y = 0.05 * total_area.h;
        let btn_h = total_area.h * 0.3;
        let btn_w = total_area.w * 0.9;
        let btn_x = total_area.x + (total_area.w - btn_w) / 2.;
        let increment_y = total_area.y + margin_y;
        let visual_y = increment_y + btn_h + margin_y;
        let decrement_y = visual_y + btn_h + margin_y;

        self.visual_rect = Rect::new(btn_x, visual_y, btn_w, btn_h);

        let increment_rect = Rect::new(btn_x, increment_y, btn_w, btn_h);
        self.increment.place(increment_rect);

        let decrement_rect = Rect::new(btn_x, decrement_y, btn_w, btn_h);
        self.decrement.place(decrement_rect);

        self.increment_dims = measure_text("+", Some(resources.font()), font_size as u16, 1.0);
        self.decrement_dims = measure_text("-", Some(resources.font()), font_size as u16, 1.0);

        self.visual_dims = measure_text("0", Some(resources.font()), font_size as u16, 1.0);
        self.font_size = font_size;
    }

    pub fn handle_input(
        &mut self,
        _: &UserInteraction,
        resources: &Resources,
        settings: &GameSettings,
    ) -> CounterWidgetInteraction {
        let mut interacted = false;
        if self
            .increment
            .handle_input(resources.sound(&SoundKind::Click), settings.volume)
        {
            if self.current < self.max {
                self.current += 1;
                interacted = true;
            }
        }

        if self
            .decrement
            .handle_input(resources.sound(&SoundKind::Click), settings.volume)
        {
            if self.current > self.min {
                self.current -= 1;
                interacted = true;
            }
        }

        return CounterWidgetInteraction {
            current: self.current,
            interacted,
        };
    }

    pub fn draw(&mut self, resources: &Resources) {
        self.increment.draw(resources);
        self.decrement.draw(resources);

        let font = resources.font();
        let mut min_dim = self.visual_rect.w.min(self.visual_rect.h);
        let mut font_size = self.font_size as u16;
        loop {
            if min_dim <= self.visual_rect.w * 0.5 {
                break;
            }

            let dims = measure_text(
                &self.current.to_string(),
                Some(&font),
                font_size as u16,
                1.0,
            );
            min_dim = dims.width.min(dims.height);
            font_size -= 1;
        }

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
