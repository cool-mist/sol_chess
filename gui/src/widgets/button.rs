use crate::{resources::Resources, widgets::*};
use macroquad::{audio::Sound, prelude::*};

pub struct ButtonWidget {
    pub is_active: bool,
    pub text: String,
    pub color: UiColor,
    is_down: bool,
    rect: Rect,
    shadow_width: f32,
}

impl ButtonWidget {
    pub fn new(is_active: bool, color: UiColor, text: impl Into<String>) -> Self {
        Self {
            is_active,
            text: text.into(),
            color,
            shadow_width: 5.0,
            ..Default::default()
        }
    }

    pub fn place(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn draw(&self, resources: &Resources) {
        self.draw_button(&self.color);
        self.draw_label(&self.text, resources, &self.color);
    }

    fn draw_button(&self, color: &UiColor) {
        let bg_color = match self.is_active {
            true => color.to_bg_color(),
            false => color.to_shadow_color(),
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

        draw_shadow(&self.rect, self.shadow_width);
    }

    fn draw_label(&self, text: &str, resources: &Resources, color: &UiColor) {
        let font_color = match self.is_active {
            true => color.to_fg_color(),
            false => Color::from_rgba(100, 100, 100, 255),
        };

        let font = resources.font();
        let target_w = self.rect.w * 0.9;
        let target_h = self.rect.h;
        let mut font_size = 20;
        let mut dims = measure_text(&text, Some(&font), font_size, 1.0);
        loop {
            if dims.width <= target_w && dims.height <= target_h {
                break;
            }

            font_size -= 2;
            dims = measure_text(&text, Some(&font), font_size as u16, 1.0);
        }

        let button_draw_offset = self.get_button_draw_offset();
        let text_params = TextParams {
            font_size,
            color: font_color,
            font: Some(&font),
            ..Default::default()
        };
        draw_text_ex(
            &text,
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

    pub fn handle_input(&mut self, click_sound: &Sound, volume: f32) -> bool {
        if !self.is_active {
            self.is_down = false;
            return false;
        }

        let (mx, my) = mouse_position();
        let c = Circle::new(mx, my, 0.0);

        if is_mouse_button_pressed(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                self.is_down = true;
                return false;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                play_sound_once(&click_sound, volume);
                self.is_down = false;
                return true;
            }

            self.is_down = false;
        }

        false
    }
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self {
            is_active: true,
            is_down: Default::default(),
            rect: Default::default(),
            shadow_width: Default::default(),
            color: UiColor::Grey,
            text: String::new(),
        }
    }
}
