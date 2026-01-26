#[allow(dead_code)]

use crate::{resources::Resources, widgets::*};
use macroquad::prelude::*;

#[derive(Default)]
pub struct IdTextInput {
    rect: Rect,
    text_rect: Rect,
    copy_box_rect: Rect,
    paste_box_rect: Rect,
}

pub struct IdTextInputDrawParams<'a> {
    pub text: &'a str,
    pub color: UiColor,
    pub copy_color: UiColor,
    pub paste_color: UiColor,
    pub shadow_width: f32,
}

impl IdTextInput {
    pub fn initialize_state() -> Self {
        Default::default()
    }

    pub fn initialize_drawables(&mut self, rect: Rect) {
        let copy_button_w = 0.25 * rect.w;
        let paste_button_h = 0.5 * rect.h;
        self.rect = rect;
        self.text_rect = Rect::new(rect.x, rect.y, rect.w - copy_button_w, rect.h);
        self.copy_box_rect = Rect::new(
            rect.x + (rect.w - copy_button_w),
            rect.y,
            copy_button_w,
            paste_button_h,
        );
        self.paste_box_rect = Rect::new(
            rect.x + (rect.w - copy_button_w),
            rect.y + paste_button_h,
            copy_button_w,
            paste_button_h,
        );
    }

    pub fn draw(&self, params: &IdTextInputDrawParams, resources: &Resources) {
        draw_rect(&self.text_rect, params.color);
        draw_rect(&self.copy_box_rect, params.copy_color);
        draw_rect(&self.paste_box_rect, params.paste_color);
        draw_shadow(&self.rect, params.shadow_width);

        let font = resources.font();
        let font_size = (0.15 * self.text_rect.w) as u16;
        let text_dims = measure_text(params.text, Some(font), font_size, 1.0);
        draw_text_ex(
            params.text,
            self.rect.x + (self.text_rect.w - text_dims.width) / 2.,
            self.rect.y + (self.text_rect.h - text_dims.height) / 2. + text_dims.offset_y,
            TextParams {
                font: Some(font),
                font_size: font_size,
                color: params.color.to_fg_color(),
                ..Default::default()
            },
        );
    }

    pub fn handle_input(&self, _: &WidgetInput) -> IdTextInputInteraction {
        IdTextInputInteraction {}
    }
}

pub struct IdTextInputInteraction {}
