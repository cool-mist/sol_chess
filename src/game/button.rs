use macroquad::prelude::*;

pub struct Button {
    pub text: String,
    pub is_pressed: bool,
    pub is_active: bool,
    pub action: ButtonAction,
    rect: Rect,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    Reset,
    Next,
}

impl Button {
    pub fn new(text: &str, x: f32, y: f32, width: f32, height: f32, action: ButtonAction) -> Self {
        let rect = Rect::new(x, y, width, height);
        Self {
            text: text.to_string(),
            is_pressed: false,
            is_active: true,
            rect,
            action,
        }
    }

    pub fn draw(&self) {
        let bg_color = Color::from_rgba(190, 190, 190, 255);
        let font_size = (self.rect.h * 0.3).floor() as u16;
        let dims = measure_text(&self.text, None, font_size, 1.0);
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg_color);

        let font_color = if self.is_active {
            Color::from_rgba(0, 0, 0, 255)
        } else {
            Color::from_rgba(100, 0, 0, 255)
        };

        draw_text(
            &self.text,
            self.rect.x + (self.rect.w - dims.width) * 0.5,
            self.rect.y + (self.rect.h - dims.height) * 0.5,
            font_size as f32,
            font_color,
        );
    }

    pub fn reset(&mut self) {
        self.is_pressed = false;
        self.is_active = true;
    }

    pub fn handle_input(&mut self) {
        if !self.is_active {
            return;
        }

        if !is_mouse_button_released(MouseButton::Left) {
            self.is_pressed = false;
            return;
        }

        let (mx, my) = mouse_position();
        let c = Circle::new(mx, my, 0.0);
        if c.overlaps_rect(&self.rect) {
            self.is_pressed = true;
            return;
        }

        self.is_pressed = false;
    }
}
