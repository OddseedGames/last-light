use macroquad::math::{vec2, Vec2};

pub const BEAM_CENTER: f32 = 235.0;
pub const BEAM_RX: f32 = 78.0;
pub const BEAM_RY: f32 = 105.0;

pub fn forward(angle: f32) -> Vec2 {
    vec2(angle.cos(), angle.sin())
}

pub fn beam_width(x: f32) -> f32 {
    if x <= 0.0 || x >= BEAM_CENTER + BEAM_RX {
        return 0.0;
    }
    let tangent = (BEAM_CENTER * BEAM_CENTER - BEAM_RX * BEAM_RX) / BEAM_CENTER;
    if x < tangent {
        x * BEAM_RY / (BEAM_CENTER * BEAM_CENTER - BEAM_RX * BEAM_RX).sqrt()
    } else {
        BEAM_RY * (1.0 - ((x - BEAM_CENTER) / BEAM_RX).powi(2)).sqrt()
    }
}

pub fn in_beam(origin: Vec2, angle: f32, point: Vec2) -> bool {
    let offset = point - origin;
    let direction = forward(angle);
    let x = offset.dot(direction);
    let y = offset.dot(vec2(-direction.y, direction.x)).abs();
    x > 0.0 && x < BEAM_CENTER + BEAM_RX && y < beam_width(x)
}

pub fn beam_strength(origin: Vec2, angle: f32, point: Vec2) -> f32 {
    if !in_beam(origin, angle, point) {
        return 0.0;
    }
    let offset = point - origin;
    let direction = forward(angle);
    let x = offset.dot(direction);
    let y = offset.dot(vec2(-direction.y, direction.x)).abs();
    let edge = ((beam_width(x) - y) / 12.0).clamp(0.0, 1.0);
    let end = ((BEAM_CENTER + BEAM_RX - x) / 22.0).clamp(0.0, 1.0);
    edge.min(end)
}
