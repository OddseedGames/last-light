use super::theme::*;
use macroquad::prelude::*;
use oddseed_last_light::{game::*, layout::*, lighting::*};

fn limb(position: Vec2, a: Vec2, b: Vec2, angle: f32, width: f32, color: Color) {
    let direction = forward(angle);
    let side = vec2(-direction.y, direction.x);
    let start = position + direction * a.x + side * a.y;
    let end = position + direction * b.x + side * b.y;
    draw_line(start.x, start.y, end.x, end.y, width, color);
    draw_circle(end.x, end.y, width / 2.0, color);
}

pub(super) fn draw_zombie(game: &Game, zombie: &Zombie) {
    let strength = beam_strength(game.player, game.angle, zombie.position);
    if strength <= 0.0 {
        return;
    }
    let position = zombie.position + ARENA;
    let direction = game.player - zombie.position;
    let angle = direction.y.atan2(direction.x);
    let skin = alpha(rgb(143, 153, 103), strength);
    let shirt = alpha(rgb(75, 93, 69), strength);
    let pants = alpha(rgb(44, 54, 48), strength);
    draw_ellipse(
        position.x - 5.0,
        position.y + 8.0,
        23.0,
        15.0,
        0.0,
        alpha(BLACK, strength * 0.45),
    );
    let step = zombie.gait.sin() * 4.0;
    limb(
        position,
        vec2(-4.0, -6.0),
        vec2(-12.0 + step, -7.0),
        angle,
        7.0,
        pants,
    );
    limb(
        position,
        vec2(-4.0, 6.0),
        vec2(-12.0 - step, 7.0),
        angle,
        7.0,
        pants,
    );
    limb(
        position,
        vec2(1.0, -10.0),
        vec2(19.0, -13.0 + step),
        angle,
        6.0,
        skin,
    );
    limb(
        position,
        vec2(0.0, 10.0),
        vec2(20.0, 10.0 - step),
        angle,
        6.0,
        skin,
    );
    draw_poly(position.x, position.y, 6, 14.0, angle.to_degrees(), shirt);
    let head = position + forward(angle) * 5.0;
    draw_circle(head.x, head.y, 8.5, skin);
    let wound = head + forward(angle + 1.2) * 5.0;
    draw_circle(wound.x, wound.y, 3.0, alpha(rgb(93, 48, 37), strength));
}

pub(super) fn draw_player(game: &Game, moving: bool) {
    let position = ARENA + game.player;
    let angle = game.angle;
    let step = if moving {
        (game.elapsed * 16.0).sin() * 4.0
    } else {
        0.0
    };
    let flashing = game.invulnerable > 0.0 && (game.elapsed * 16.0).sin() > 0.0;
    let color = if flashing { DANGER } else { rgb(151, 166, 157) };
    draw_ellipse(
        position.x - 3.0,
        position.y + 7.0,
        21.0,
        15.0,
        0.0,
        alpha(BLACK, 0.65),
    );
    limb(
        position,
        vec2(-3.0, -6.0),
        vec2(-12.0 + step, -7.0),
        angle,
        7.0,
        rgb(54, 67, 67),
    );
    limb(
        position,
        vec2(-3.0, 6.0),
        vec2(-12.0 - step, 7.0),
        angle,
        7.0,
        rgb(54, 67, 67),
    );
    limb(
        position,
        vec2(0.0, -9.0),
        vec2(18.0, -6.0),
        angle,
        7.0,
        color,
    );
    limb(
        position,
        vec2(0.0, 9.0),
        vec2(12.0, -2.0),
        angle,
        7.0,
        color,
    );
    draw_poly(
        position.x,
        position.y,
        6,
        14.0,
        angle.to_degrees(),
        rgb(69, 91, 89),
    );
    limb(
        position,
        vec2(-6.0, -6.0),
        vec2(-6.0, 6.0),
        angle,
        5.0,
        rgb(43, 58, 56),
    );
    let head = position + forward(angle) * 2.5;
    draw_circle(head.x, head.y, 8.0, rgb(201, 184, 147));
    draw_circle(head.x - 2.0, head.y - 2.0, 6.0, color);
    let recoil = if game.cooldown > SHOT_DELAY - 0.10 {
        -4.0
    } else {
        0.0
    };
    limb(
        position,
        vec2(10.0, -4.0),
        vec2(31.0 + recoil, -4.0),
        angle,
        5.0,
        rgb(37, 42, 40),
    );
    limb(
        position,
        vec2(18.0, -4.0),
        vec2(26.0 + recoil, -4.0),
        angle,
        3.0,
        rgb(119, 116, 91),
    );
    let lamp = position + forward(angle) * 18.0;
    draw_circle(lamp.x, lamp.y, 3.0, rgb(239, 225, 159));
    if game.reload_left > 0.0 {
        draw_rectangle(position.x - 20.0, position.y + 26.0, 40.0, 3.0, LINE);
        draw_rectangle(
            position.x - 20.0,
            position.y + 26.0,
            40.0 * (1.0 - game.reload_left / RELOAD_TIME),
            3.0,
            AMBER,
        );
    }
}
