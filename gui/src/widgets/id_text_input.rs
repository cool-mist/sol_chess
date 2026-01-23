use crate::widgets::*;
use macroquad::prelude::*;

pub struct IdTextInput {
    rect: Rect,
    text_rect: Rect,
    copy_box_rect: Rect,
    paste_box_rect: Rect,
    shadow_width: f32,
    color: UiColor,
    copy_color: UiColor,
    paste_color: UiColor,
}

impl IdTextInput {
    pub fn new(rect: Rect) -> Self {
        let copy_button_w = 0.25 * rect.w;
        let paste_button_h = 0.5 * rect.h;
        Self {
            rect,
            text_rect: Rect::new(rect.x, rect.y, rect.w - copy_button_w, rect.h),
            copy_box_rect: Rect::new(
                rect.x + (rect.w - copy_button_w),
                rect.y,
                copy_button_w,
                paste_button_h,
            ),
            paste_box_rect: Rect::new(
                rect.x + (rect.w - copy_button_w),
                rect.y + paste_button_h,
                copy_button_w,
                paste_button_h,
            ),
            shadow_width: 3.0,
            color: UiColor::Yellow,
            copy_color: UiColor::Blue,
            paste_color: UiColor::Pink,
        }
    }

    pub fn draw(&self, font: &Font, text: &str) {
        draw_rect(&self.text_rect, self.color);
        draw_rect(&self.copy_box_rect, self.copy_color);
        draw_rect(&self.paste_box_rect, self.paste_color);
        draw_shadow(&self.rect, self.shadow_width);

        let font_size = (0.15 * self.text_rect.w) as u16;
        let text_dims = measure_text(text, Some(font), font_size, 1.0);
        draw_text_ex(
            &text,
            self.rect.x + (self.text_rect.w - text_dims.width) / 2.,
            self.rect.y + (self.text_rect.h - text_dims.height) / 2. + text_dims.offset_y,
            TextParams {
                font: Some(font),
                font_size: font_size,
                color: self.color.to_fg_color(),
                ..Default::default()
            },
        );
    }
}

impl Default for IdTextInput {
    fn default() -> Self {
        Self {
            rect: Default::default(),
            text_rect: Default::default(),
            copy_box_rect: Default::default(),
            paste_box_rect: Default::default(),
            shadow_width: Default::default(),
            color: UiColor::Grey,
            copy_color: UiColor::Grey,
            paste_color: UiColor::Grey,
        }
    }
}
