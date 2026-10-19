use crate::{audio::Audio, browser, view};
use macroquad::prelude::*;
use oddseed_last_light::{
    game::Game,
    input::{InputController, RawInput},
    layout::*,
};

pub fn config() -> Conf {
    Conf {
        window_title: "Последний луч · OddseedGames".to_owned(),
        window_width: W as i32,
        window_height: H as i32,
        high_dpi: true,
        ..Default::default()
    }
}

pub async fn run() {
    let font = view::load_font();
    let ground = view::world::material();
    let preview = view::world::preview();
    let mut audio = Audio::load().await;
    let mut game = Game::new((get_time() * 1_000_000.0) as u64 + 7391);
    let mut controls = InputController::new(RawInput::capture().pointer);
    let mut best = 0;
    browser::ready();
    loop {
        browser::pause_if_requested(&mut game);
        let frame = controls.update(&mut game, RawInput::capture());
        if frame.toggle_sound {
            audio.enabled = !audio.enabled;
        }
        let before = (game.fired, game.health, game.reload_left);
        game.update(get_frame_time(), frame.controls);
        best = best.max(game.kills);
        audio.play_changes(&game, before);
        view::draw(
            &game,
            &preview,
            &ground,
            &font,
            audio.enabled,
            best,
            frame.pointer,
            frame.controls.movement.length_squared() > 0.0,
        );
        browser::report(&game);
        next_frame().await;
    }
}
