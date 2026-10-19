use crate::game::{HEIGHT, WIDTH};
use macroquad::math::{vec2, Rect, Vec2};

pub const W: f32 = 1280.0;
pub const H: f32 = 880.0;
pub const ARENA: Vec2 = vec2(28.0, 110.0);
pub const SOUND_BUTTON: Rect = Rect::new(1025.0, 37.0, 132.0, 35.0);
pub const PAUSE_BUTTON: Rect = Rect::new(1168.0, 37.0, 84.0, 35.0);
pub const START_BUTTON: Rect = Rect::new(88.0, 537.0, 400.0, 50.0);
pub const RESUME_BUTTON: Rect = Rect::new(399.0, 466.0, 482.0, 50.0);

pub fn arena_rect() -> Rect {
    Rect::new(ARENA.x, ARENA.y, WIDTH, HEIGHT)
}
