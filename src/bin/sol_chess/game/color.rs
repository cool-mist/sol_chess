use macroquad::prelude::*;

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
            UiColor::Pink => Color::from_rgba(234, 128, 71, 200),
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
