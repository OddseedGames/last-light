use crate::{
    game::{Game, Input, Phase},
    layout::*,
};
use macroquad::prelude::*;

#[derive(Default)]
pub struct RawInput {
    pub pointer: Vec2,
    pub movement: Vec2,
    pub turn: f32,
    pub mouse_pressed: bool,
    pub mouse_held: bool,
    pub space_held: bool,
    pub enter: bool,
    pub pause: bool,
    pub sound: bool,
    pub restart: bool,
    pub reload: bool,
}

impl RawInput {
    pub fn capture() -> Self {
        let scale = (screen_width() / W).min(screen_height() / H);
        let offset = vec2(
            (screen_width() - W * scale) / 2.0,
            (screen_height() - H * scale) / 2.0,
        );
        Self {
            pointer: (Vec2::from(mouse_position()) - offset) / scale,
            movement: vec2(
                (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) as u8 as f32
                    - (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) as u8 as f32,
                (is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)) as u8 as f32
                    - (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)) as u8 as f32,
            ),
            turn: is_key_down(KeyCode::E) as u8 as f32 - is_key_down(KeyCode::Q) as u8 as f32,
            mouse_pressed: is_mouse_button_pressed(MouseButton::Left),
            mouse_held: is_mouse_button_down(MouseButton::Left),
            space_held: is_key_down(KeyCode::Space),
            enter: is_key_pressed(KeyCode::Enter),
            pause: is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P),
            sound: is_key_pressed(KeyCode::M),
            restart: is_key_pressed(KeyCode::N),
            reload: is_key_pressed(KeyCode::R),
        }
    }
}

pub struct FrameInput {
    pub controls: Input,
    pub toggle_sound: bool,
    pub pointer: Vec2,
}

pub struct InputController {
    last_mouse: Vec2,
    mouse_aim: bool,
    fire_armed: bool,
}

impl InputController {
    pub fn new(pointer: Vec2) -> Self {
        Self {
            last_mouse: pointer,
            mouse_aim: true,
            fire_armed: true,
        }
    }

    pub fn update(&mut self, game: &mut Game, raw: RawInput) -> FrameInput {
        let mut consumed = false;
        let toggle_sound = raw.sound || (raw.mouse_pressed && SOUND_BUTTON.contains(raw.pointer));
        if toggle_sound {
            consumed = true;
        }
        if raw.pause || (raw.mouse_pressed && PAUSE_BUTTON.contains(raw.pointer)) {
            game.toggle_pause();
            consumed = true;
        }
        let overlay = if game.phase == Phase::Ready {
            START_BUTTON
        } else {
            RESUME_BUTTON
        };
        if game.phase != Phase::Playing
            && (raw.enter || (raw.mouse_pressed && overlay.contains(raw.pointer)))
        {
            if game.phase == Phase::Paused {
                game.toggle_pause();
            } else {
                game.start();
                self.mouse_aim = true;
            }
            consumed = true;
        }
        if game.phase == Phase::Paused && raw.restart {
            game.start();
        }
        if consumed {
            self.fire_armed = false;
        }
        if !raw.mouse_held && !raw.space_held {
            self.fire_armed = true;
        }
        if raw.pointer.distance(self.last_mouse) > 0.25 {
            self.mouse_aim = true;
        }
        if raw.turn != 0.0 {
            self.mouse_aim = false;
        }
        let target = raw.pointer - ARENA - game.player;
        let aim =
            (self.mouse_aim && target.length_squared() > 1.0).then(|| target.y.atan2(target.x));
        self.last_mouse = raw.pointer;
        FrameInput {
            pointer: raw.pointer,
            toggle_sound,
            controls: Input {
                movement: raw.movement,
                aim,
                turn: raw.turn,
                fire: self.fire_armed
                    && !consumed
                    && ((raw.mouse_held && arena_rect().contains(raw.pointer)) || raw.space_held),
                reload: raw.reload,
            },
        }
    }
}
