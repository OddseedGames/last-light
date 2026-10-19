mod actors;
pub mod hud;
pub mod overlay;
mod theme;
pub mod world;

use macroquad::prelude::*;
use oddseed_last_light::{game::*, layout::*};

fn camera() -> Camera2D {
    let scale = (screen_width() / W).min(screen_height() / H);
    Camera2D {
        target: vec2(W / 2.0, H / 2.0),
        zoom: vec2(2.0 * scale / screen_width(), 2.0 * scale / screen_height()),
        ..Default::default()
    }
}

fn clip_world(enabled: bool) {
    let dpi = screen_dpi_scale();
    let scale = (screen_width() / W).min(screen_height() / H) * dpi;
    let offset = vec2(
        (screen_width() * dpi - W * scale) / 2.0,
        (screen_height() * dpi - H * scale) / 2.0,
    );
    let clip = enabled.then(|| {
        (
            (offset.x + ARENA.x * scale).round() as i32,
            (offset.y + ARENA.y * scale).round() as i32,
            (WIDTH * scale).round() as i32,
            (HEIGHT * scale).round() as i32,
        )
    });
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

pub fn load_font() -> Font {
    let font =
        load_ttf_font_from_bytes(include_bytes!("../../assets/IBMPlexMono-Regular.ttf")).unwrap();
    let characters: Vec<char> = (' '..='~')
        .chain('А'..='я')
        .chain("Ёё.…·".chars())
        .collect();
    font.populate_font_cache(
        &characters,
        (theme::FONT_SIZE as f32 * screen_dpi_scale()).ceil() as u16,
    );
    font
}

pub fn draw(
    game: &Game,
    preview: &Game,
    material: &Material,
    font: &Font,
    audio: bool,
    best: u32,
    point: Vec2,
    moving: bool,
) {
    clear_background(theme::rgb(9, 16, 14));
    set_camera(&camera());
    clip_world(true);
    world::draw(
        if game.phase == Phase::Ready {
            preview
        } else {
            game
        },
        material,
        moving && game.phase == Phase::Playing,
    );
    clip_world(false);
    hud::draw(game, font, audio, best, point);
    overlay::draw(game, font, point);
    overlay::reticle(game, point);
    set_default_camera();
}
