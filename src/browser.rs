use oddseed_last_light::game::{Game, Phase};
use std::sync::atomic::{AtomicBool, Ordering};

static PAUSE_REQUEST: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn last_light_ready();
    fn last_light_status(
        phase: u32,
        elapsed: f32,
        kills: u32,
        health: u32,
        ammo: u32,
        reload: f32,
        x: f32,
        y: f32,
        angle: f32,
        visible: u32,
        fired: u32,
    );
}

#[no_mangle]
pub extern "C" fn last_light_pause() {
    PAUSE_REQUEST.store(true, Ordering::Relaxed);
}

pub fn pause_if_requested(game: &mut Game) {
    if PAUSE_REQUEST.swap(false, Ordering::Relaxed) && game.phase == Phase::Playing {
        game.toggle_pause();
    }
}

pub fn ready() {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        last_light_ready();
    }
}

pub fn report(_game: &Game) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        last_light_status(
            _game.phase as u32,
            _game.elapsed,
            _game.kills,
            _game.health as u32,
            _game.ammo as u32,
            _game.reload_left,
            _game.player.x,
            _game.player.y,
            _game.angle,
            _game
                .zombies
                .iter()
                .filter(|zombie| _game.visible(zombie))
                .count() as u32,
            _game.fired,
        );
    }
}
