use macroquad::math::{vec2, Vec2};
use oddseed_last_light::{
    game::{Game, Phase},
    input::{InputController, RawInput},
    layout::*,
};

#[test]
fn clicking_start_requires_release_before_shooting() {
    let mut game = Game::new(1);
    let pointer = START_BUTTON.center();
    let mut controller = InputController::new(pointer);
    let started = controller.update(
        &mut game,
        RawInput {
            pointer,
            mouse_pressed: true,
            mouse_held: true,
            ..RawInput::default()
        },
    );
    assert_eq!(game.phase, Phase::Playing);
    assert!(!started.controls.fire);
    let held = controller.update(
        &mut game,
        RawInput {
            pointer,
            mouse_held: true,
            ..RawInput::default()
        },
    );
    assert!(!held.controls.fire);
    controller.update(
        &mut game,
        RawInput {
            pointer,
            ..RawInput::default()
        },
    );
    let shot = controller.update(
        &mut game,
        RawInput {
            pointer,
            mouse_pressed: true,
            mouse_held: true,
            ..RawInput::default()
        },
    );
    assert!(shot.controls.fire);
}

#[test]
fn keyboard_aim_stays_active_until_the_mouse_moves() {
    let mut game = Game::new(1);
    game.start();
    let pointer = ARENA + game.player + vec2(180.0, 0.0);
    let mut controller = InputController::new(pointer);
    let mouse = controller.update(
        &mut game,
        RawInput {
            pointer,
            ..RawInput::default()
        },
    );
    assert_eq!(mouse.controls.aim, Some(0.0));
    let turning = controller.update(
        &mut game,
        RawInput {
            pointer,
            turn: -1.0,
            ..RawInput::default()
        },
    );
    assert_eq!(turning.controls.aim, None);
    assert_eq!(turning.controls.turn, -1.0);
    let stationary = controller.update(
        &mut game,
        RawInput {
            pointer,
            ..RawInput::default()
        },
    );
    assert_eq!(stationary.controls.aim, None);
    let moved = controller.update(
        &mut game,
        RawInput {
            pointer: pointer + vec2(0.0, 50.0),
            ..RawInput::default()
        },
    );
    assert!(moved.controls.aim.unwrap() > 0.2);
}

#[test]
fn hud_buttons_do_not_trigger_a_shot() {
    let mut game = Game::new(1);
    game.start();
    let mut controller = InputController::new(Vec2::ZERO);
    let sound = controller.update(
        &mut game,
        RawInput {
            pointer: SOUND_BUTTON.center(),
            mouse_pressed: true,
            mouse_held: true,
            ..RawInput::default()
        },
    );
    assert!(sound.toggle_sound);
    assert!(!sound.controls.fire);
    let pause = controller.update(
        &mut game,
        RawInput {
            pointer: PAUSE_BUTTON.center(),
            mouse_pressed: true,
            mouse_held: true,
            ..RawInput::default()
        },
    );
    assert_eq!(game.phase, Phase::Paused);
    assert!(!pause.controls.fire);
}
