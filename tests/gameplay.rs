use macroquad::math::{vec2, Vec2};
use oddseed_last_light::{game::*, lighting::*};
use std::f32::consts::FRAC_PI_2;

fn playing() -> Game {
    let mut game = Game::new(7391);
    game.start();
    game.angle = 0.0;
    game
}

fn zombie(game: &mut Game, offset: Vec2) {
    game.zombies.push(Zombie {
        position: game.player + offset,
        speed: 55.0,
        gait: 0.0,
    });
}

fn advance(game: &mut Game, seconds: f32, input: Input) {
    for _ in 0..(seconds * 120.0).ceil() as usize {
        game.update(1.0 / 120.0, input);
    }
}

#[test]
fn flashlight_is_finite_and_excludes_sides_and_back() {
    let origin = vec2(100.0, 100.0);
    for offset in [vec2(40.0, 0.0), vec2(200.0, 60.0), vec2(280.0, 25.0)] {
        assert!(in_beam(origin, 0.0, origin + offset));
    }
    for offset in [
        vec2(-20.0, 0.0),
        vec2(0.0, 30.0),
        vec2(180.0, 100.0),
        vec2(314.0, 0.0),
    ] {
        assert!(!in_beam(origin, 0.0, origin + offset));
        assert_eq!(beam_strength(origin, 0.0, origin + offset), 0.0);
    }
}

#[test]
fn beam_expands_then_ends_with_an_oval_cap() {
    assert!(beam_width(180.0) > beam_width(60.0));
    assert!((beam_width(BEAM_CENTER) - BEAM_RY).abs() < 0.001);
    assert!(beam_width(285.0) < beam_width(250.0));
    assert_eq!(beam_width(BEAM_CENTER + BEAM_RX), 0.0);
    let tangent = (BEAM_CENTER.powi(2) - BEAM_RX.powi(2)) / BEAM_CENTER;
    assert!((beam_width(tangent - 0.01) - beam_width(tangent + 0.01)).abs() < 0.02);
}

#[test]
fn rotating_light_reveals_a_previously_hidden_zombie() {
    let mut game = playing();
    zombie(&mut game, vec2(0.0, 180.0));
    assert!(!game.visible(&game.zombies[0]));
    game.angle = FRAC_PI_2;
    assert!(game.visible(&game.zombies[0]));
    assert!(game.fire());
    assert_eq!(game.kills, 1);
    assert!(game.zombies.is_empty());
}

#[test]
fn one_hit_kills_a_lit_zombie() {
    let mut game = playing();
    zombie(&mut game, vec2(180.0, 0.0));
    assert!(game.fire());
    assert_eq!(game.kills, 1);
    assert_eq!(game.ammo, MAGAZINE - 1);
    assert!(game.zombies.is_empty());
    assert_eq!(game.remains.len(), 1);
}

#[test]
fn pellets_cannot_damage_zombies_in_darkness_even_on_the_firing_axis() {
    let mut game = playing();
    zombie(&mut game, vec2(335.0, 0.0));
    zombie(&mut game, vec2(-100.0, 0.0));
    zombie(&mut game, vec2(160.0, 110.0));
    assert!(game.fire());
    assert_eq!(game.kills, 0);
    assert_eq!(game.zombies.len(), 3);
}

#[test]
fn illuminated_targets_outside_shotgun_spread_survive() {
    let mut game = playing();
    zombie(&mut game, vec2(180.0, 75.0));
    assert!(game.visible(&game.zombies[0]));
    game.fire();
    assert_eq!(game.kills, 0);
}

#[test]
fn a_spread_can_kill_several_zombies() {
    let mut game = playing();
    for y in [-35.0, 0.0, 35.0] {
        zombie(&mut game, vec2(180.0, y));
    }
    game.fire();
    assert_eq!(game.kills, 3);
    assert!(game.zombies.is_empty());
}

#[test]
fn shotgun_has_a_cooldown_and_automatically_reloads_after_six_shells() {
    let mut game = playing();
    assert!(game.fire());
    assert!(!game.fire());
    for _ in 1..MAGAZINE {
        advance(&mut game, SHOT_DELAY + 0.01, Input::default());
        assert!(game.fire());
    }
    assert_eq!(game.ammo, 0);
    assert!(game.reload_left > 0.0);
    assert!(!game.fire());
    advance(&mut game, RELOAD_TIME + 0.01, Input::default());
    assert_eq!(game.ammo, MAGAZINE);
    assert_eq!(game.reload_left, 0.0);
    assert!(game.fire());
}

#[test]
fn manual_reload_blocks_firing_until_it_finishes() {
    let mut game = playing();
    game.reload();
    assert_eq!(game.reload_left, 0.0);
    game.fire();
    game.reload();
    advance(&mut game, RELOAD_TIME / 2.0, Input::default());
    assert!(!game.fire());
    assert_eq!(game.ammo, MAGAZINE - 1);
    advance(&mut game, RELOAD_TIME / 2.0 + 0.02, Input::default());
    assert_eq!(game.ammo, MAGAZINE);
}

#[test]
fn diagonal_movement_is_not_faster_and_player_stays_inside_arena() {
    let mut straight = playing();
    let mut diagonal = playing();
    let start = straight.player;
    advance(
        &mut straight,
        0.25,
        Input {
            movement: vec2(1.0, 0.0),
            ..Input::default()
        },
    );
    advance(
        &mut diagonal,
        0.25,
        Input {
            movement: vec2(1.0, 1.0),
            ..Input::default()
        },
    );
    assert!((straight.player.distance(start) - diagonal.player.distance(start)).abs() < 0.01);
    diagonal.player = vec2(WIDTH - 25.0, HEIGHT - 25.0);
    advance(
        &mut diagonal,
        0.25,
        Input {
            movement: vec2(1.0, 1.0),
            ..Input::default()
        },
    );
    assert_eq!(diagonal.player, vec2(WIDTH - 24.0, HEIGHT - 24.0));
}

#[test]
fn all_spawn_sides_are_outside_the_arena() {
    let mut game = playing();
    let mut sides = [false; 4];
    for _ in 0..100 {
        game.spawn();
        let p = game.zombies.last().unwrap().position;
        assert!(p.x < 0.0 || p.y < 0.0 || p.x > WIDTH || p.y > HEIGHT);
        sides[0] |= p.y < 0.0;
        sides[1] |= p.x > WIDTH;
        sides[2] |= p.y > HEIGHT;
        sides[3] |= p.x < 0.0;
    }
    assert!(sides.into_iter().all(|side| side));
}

#[test]
fn invisible_zombies_still_chase_the_player() {
    let mut game = playing();
    zombie(&mut game, vec2(-200.0, 0.0));
    let distance = game.zombies[0].position.distance(game.player);
    advance(&mut game, 0.25, Input::default());
    assert!(!game.visible(&game.zombies[0]));
    assert!(game.zombies[0].position.distance(game.player) < distance - 10.0);
}

#[test]
fn hidden_zombies_can_bite_and_invulnerability_prevents_stacked_damage() {
    let mut game = playing();
    zombie(&mut game, vec2(-20.0, 0.0));
    zombie(&mut game, vec2(-21.0, 0.0));
    game.update(0.01, Input::default());
    assert_eq!(game.health, 2);
    assert!(game.invulnerable > 1.4);
    assert_eq!(game.phase, Phase::Playing);
    for expected_health in [1, 0] {
        game.invulnerable = 0.0;
        game.zombies[0].position = game.player - vec2(20.0, 0.0);
        game.update(0.01, Input::default());
        assert_eq!(game.health, expected_health);
    }
    assert_eq!(game.phase, Phase::Dead);
    assert!(!game.fire());
}

#[test]
fn pause_freezes_movement_pursuit_timers_reload_and_shooting() {
    let mut game = playing();
    zombie(&mut game, vec2(-180.0, 0.0));
    game.fire();
    game.reload();
    game.toggle_pause();
    let position = game.player;
    let enemy = game.zombies[0].position;
    let reload = game.reload_left;
    let cooldown = game.cooldown;
    advance(
        &mut game,
        3.0,
        Input {
            movement: Vec2::ONE,
            fire: true,
            turn: 1.0,
            ..Input::default()
        },
    );
    assert_eq!(game.phase, Phase::Paused);
    assert_eq!(game.player, position);
    assert_eq!(game.zombies[0].position, enemy);
    assert_eq!(game.elapsed, 0.0);
    assert_eq!(game.angle, 0.0);
    assert_eq!(game.reload_left, reload);
    assert_eq!(game.cooldown, cooldown);
    assert_eq!(game.fired, 1);
    assert!(!game.fire());
    game.toggle_pause();
    advance(&mut game, 0.1, Input::default());
    assert!(game.elapsed > 0.0);
}

#[test]
fn restart_restores_the_entire_round() {
    let mut game = playing();
    zombie(&mut game, vec2(180.0, 0.0));
    game.fire();
    game.health = 0;
    game.phase = Phase::Dead;
    game.start();
    assert_eq!(game.phase, Phase::Playing);
    assert_eq!(game.health, 3);
    assert_eq!(game.ammo, MAGAZINE);
    assert_eq!(game.kills, 0);
    assert_eq!(game.fired, 0);
    assert_eq!(game.elapsed, 0.0);
    assert!(game.zombies.is_empty());
    assert!(game.remains.is_empty());
    assert!(game.shots.is_empty());
    assert_eq!(game.reload_left, 0.0);
}
