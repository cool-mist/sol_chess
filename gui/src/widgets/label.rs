use macroquad::prelude::*;

use crate::resources::Resources;

#[derive(Default)]
pub struct LabelWidget {
    rect: Rect,
    font_size: f32,
    text_dimensions: TextDimensions,
    text: String,
}

impl LabelWidget {
    pub fn initialize_state(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
    }

    pub fn initialize_drawables(&mut self, rect: Rect, font_size: f32, resources: &Resources) {
        self.rect = rect;
        self.font_size = font_size;
        self.text_dimensions = measure_text(&self.text, Some(resources.font()), font_size as u16, 1.0);
    }

    pub fn draw(&self, resources: &Resources) {
        let draw_text_params = TextParams {
            font: Some(&resources.font()),
            font_size: self.font_size as u16,
            color: BLACK,
            ..Default::default()
        };

        draw_text_ex(
            &self.text,
            self.rect.x,
            self.rect.y + self.text_dimensions.offset_y,
            draw_text_params,
        );
    }
}
