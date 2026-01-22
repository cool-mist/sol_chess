use macroquad::prelude::*;

pub fn draw_shadow(rect: Rect, shadow_width: f32) {
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
