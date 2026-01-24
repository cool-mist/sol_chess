mod button;
pub use button::*;

mod id_text_input;
pub use id_text_input::*;

use macroquad::{
    audio::{self, Sound},
    prelude::*,
};
use quad_snd::PlaySoundParams;

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

pub fn draw_rect(rect: &Rect, color: UiColor) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color.to_bg_color());
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
