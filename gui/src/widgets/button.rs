use crate::{resources::Resources, widgets::*};
use macroquad::{audio::Sound, prelude::*};

pub struct ButtonWidget {
    pub is_active: bool,
    is_down: bool,
    rect: Rect,
    shadow_width: f32,
}

pub struct ButtonInteraction {
    pub is_clicked: bool,
}

impl ButtonWidget {
    pub fn initialize_state(is_active: bool) -> Self {
        Self {
            is_active,
            ..Default::default()
        }
    }

    pub fn initialize_drawables(&mut self, rect: Rect) {
        self.rect = rect;
        self.shadow_width = 5.0;
    }

    pub fn draw(&self, text: &str, color: &UiColor, resources: &Resources) {
        self.draw_button(color);
        self.draw_label(text, resources, color);
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
        let mut dim_width = self.rect.w;
        let mut font_size = 30.0;
        let mut dims = measure_text(&text, Some(&font), font_size as u16, 1.0);
        loop {
            if dim_width <= self.rect.w * 0.8 {
                break;
            }

            dims = measure_text(&text, Some(&font), font_size as u16, 1.0);
            dim_width = dims.width;
            font_size -= 2.0;
        }

        let button_draw_offset = self.get_button_draw_offset();

        let text_params = TextParams {
            font_size: font_size as u16,
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

    pub fn handle_input(&mut self, click_sound: &Sound, volume: f32) -> ButtonInteraction {
        if !self.is_active {
            self.is_down = false;
            return ButtonInteraction { is_clicked: false };
        }

        let (mx, my) = mouse_position();
        let c = Circle::new(mx, my, 0.0);

        if is_mouse_button_pressed(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                self.is_down = true;
                return ButtonInteraction { is_clicked: false };
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                play_sound_once(&click_sound, volume);
                self.is_down = false;
                return ButtonInteraction { is_clicked: true };
            }

            self.is_down = false;
        }

        return ButtonInteraction { is_clicked: false };
    }
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self {
            is_active: true,
            is_down: Default::default(),
            rect: Default::default(),
            shadow_width: Default::default(),
        }
    }
}
