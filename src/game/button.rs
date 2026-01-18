use std::sync::Arc;

use macroquad::{audio::Sound, prelude::*};

use crate::game::sound::Sounds;

use super::{color::UiColor, shadow::draw_shadow};

pub struct Button {
    pub is_active: bool,
    pub text: String,
    is_down: bool,
    is_clicked: bool,
    rect: Rect,
    shadow_width: f32,
    pub color: UiColor,
    sound: Sound,
    font: Arc<Font>,
}

impl Button {
    pub fn new(text: &str, rect: Rect, color: UiColor, sound: Sound, font: Arc<Font>) -> Self {
        Self {
            text: text.to_string(),
            is_down: false,
            is_clicked: false,
            is_active: true,
            rect,
            shadow_width: 5.0,
            color,
            sound,
            font,
        }
    }

    pub fn is_clicked(&mut self) -> bool {
        if self.is_clicked {
            self.is_clicked = false;
            return true;
        }

        false
    }

    pub fn draw(&self) {
        self.draw_button();
        self.draw_label();
    }

    fn draw_button(&self) {
        let bg_color = match self.is_active {
            true => self.color.to_bg_color(),
            false => self.color.to_shadow_color(),
        };
        let button_draw_offset = self.get_button_draw_offset();
        draw_rectangle(
            self.rect.x + button_draw_offset,
            self.rect.y + button_draw_offset,
            self.rect.w,
            self.rect.h,
            bg_color,
        );

        self.draw_shadow();
    }

    fn draw_shadow(&self) {
        if !self.is_active {
            return;
        }

        if self.is_down {
            return;
        }

        draw_shadow(self.rect, self.shadow_width);
    }

    fn draw_label(&self) {
        let font_color = match self.is_active {
            true => self.color.to_fg_color(),
            false => Color::from_rgba(100, 100, 100, 255),
        };

        let font_size = (0.2 * self.rect.w) as u16;
        let dims = measure_text(&self.text, Some(&self.font), font_size, 1.0);
        let button_draw_offset = self.get_button_draw_offset();

        let text_params = TextParams {
            font_size: font_size as u16,
            color: font_color,
            font: Some(&self.font),
            ..Default::default()
        };
        draw_text_ex(
            &self.text,
            self.rect.x + (self.rect.w - dims.width) * 0.5 + button_draw_offset,
            self.rect.y + (self.rect.h - dims.height) * 0.5 + dims.offset_y + button_draw_offset,
            text_params,
        );
    }

    fn get_button_draw_offset(&self) -> f32 {
        let button_pressed_correction = match self.is_down {
            true => self.shadow_width,
            false => match self.is_active {
                true => 0.0,
                false => self.shadow_width,
            },
        };
        button_pressed_correction
    }

    pub fn handle_input(&mut self) {
        if !self.is_active {
            self.is_down = false;
            return;
        }

        let (mx, my) = mouse_position();
        let c = Circle::new(mx, my, 0.0);

        if is_mouse_button_pressed(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                self.is_down = true;
                return;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                self.is_clicked = true;
                Sounds::play(&self.sound);
                self.is_down = false;
                return;
            }

            self.is_down = false;
        }
    }
}
