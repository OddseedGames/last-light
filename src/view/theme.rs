use macroquad::prelude::*;

pub(super) const INK: Color = Color::new(0.89, 0.90, 0.83, 1.0);
pub(super) const MUTED: Color = Color::new(0.43, 0.51, 0.49, 1.0);
pub(super) const AMBER: Color = Color::new(0.91, 0.73, 0.40, 1.0);
pub(super) const LINE: Color = Color::new(0.15, 0.20, 0.19, 1.0);
pub(super) const DANGER: Color = Color::new(0.87, 0.35, 0.28, 1.0);
pub(super) const FONT_SIZE: u16 = 64;

pub(super) fn alpha(mut color: Color, value: f32) -> Color {
    color.a *= value;
    color
}

pub(super) fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}

pub(super) fn txt(font: &Font, content: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text_ex(
        content,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: FONT_SIZE,
            font_scale: size / FONT_SIZE as f32,
            color,
            ..Default::default()
        },
    );
}

pub(super) fn right(font: &Font, content: &str, x: f32, y: f32, size: f32, color: Color) {
    let width = measure_text(content, Some(font), FONT_SIZE, size / FONT_SIZE as f32).width;
    txt(font, content, x - width, y, size, color);
}

pub(super) fn panel(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, LINE);
}

pub(super) fn button(font: &Font, label: &str, rect: Rect, pointer: Vec2) {
    let color = if rect.contains(pointer) { INK } else { AMBER };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    txt(
        font,
        label,
        rect.x + 20.0,
        rect.y + 30.0,
        15.0,
        rgb(17, 23, 21),
    );
}

pub(super) fn time_label(seconds: f32) -> String {
    format!("{:02}:{:02}", seconds as u32 / 60, seconds as u32 % 60)
}
