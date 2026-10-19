use super::theme::*;
use macroquad::prelude::*;
use oddseed_last_light::{game::*, layout::*, lighting::*};

use super::actors::{draw_player, draw_zombie};

pub fn material() -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: "#version 100\nattribute vec3 position;\nvarying vec2 world;\nuniform mat4 Model;\nuniform mat4 Projection;\nvoid main() { world = position.xy; gl_Position = Projection * Model * vec4(position, 1.0); }",
            fragment: include_str!("../ground.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Player", UniformType::Float4),
                UniformDesc::new("Beam", UniformType::Float4),
            ],
            ..Default::default()
        },
    )
    .expect("flashlight shader")
}

fn draw_ground(game: &Game, material: &Material) {
    material.set_uniform(
        "Player",
        (game.player.x, game.player.y, game.angle, game.elapsed),
    );
    material.set_uniform(
        "Beam",
        (
            BEAM_CENTER,
            BEAM_RX,
            BEAM_RY,
            (BEAM_CENTER * BEAM_CENTER - BEAM_RX * BEAM_RX) / BEAM_CENTER,
        ),
    );
    gl_use_material(material);
    draw_rectangle(ARENA.x, ARENA.y, WIDTH, HEIGHT, WHITE);
    gl_use_default_material();
}

pub fn draw(game: &Game, material: &Material, moving: bool) {
    draw_ground(game, material);
    for remains in &game.remains {
        let strength = beam_strength(game.player, game.angle, remains.position);
        if strength > 0.0 {
            let p = remains.position + ARENA;
            let fade = strength * (1.0 - remains.age / 8.0);
            draw_ellipse(
                p.x,
                p.y,
                23.0,
                15.0,
                remains.angle.to_degrees(),
                alpha(rgb(79, 39, 31), fade),
            );
            draw_ellipse(
                p.x,
                p.y,
                15.0,
                9.0,
                remains.angle.to_degrees(),
                alpha(rgb(58, 68, 47), fade),
            );
            for i in 0..5 {
                let offset = forward(remains.angle + i as f32 * 1.7) * (16.0 + i as f32 * 2.0);
                draw_circle(
                    p.x + offset.x,
                    p.y + offset.y,
                    2.0,
                    alpha(rgb(89, 43, 32), fade),
                );
            }
        }
    }
    for zombie in &game.zombies {
        draw_zombie(game, zombie);
    }
    for shot in &game.shots {
        let intensity = 1.0 - shot.age / 0.16;
        let start = shot.origin + ARENA + forward(game.angle) * 30.0;
        for end in &shot.ends {
            let end = *end + ARENA;
            draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                1.0,
                alpha(AMBER, intensity * 0.60),
            );
        }
        if shot.age < 0.065 {
            draw_circle(start.x, start.y, 18.0 * intensity, alpha(AMBER, 0.20));
            draw_poly(
                start.x,
                start.y,
                5,
                11.0 * intensity,
                game.angle.to_degrees(),
                rgb(255, 237, 167),
            );
        }
    }
    draw_player(game, moving);
    draw_rectangle_lines(ARENA.x, ARENA.y, WIDTH, HEIGHT, 1.0, LINE);
    for p in [
        ARENA,
        ARENA + vec2(WIDTH, 0.0),
        ARENA + vec2(0.0, HEIGHT),
        ARENA + vec2(WIDTH, HEIGHT),
    ] {
        let sx = if p.x > W / 2.0 { -1.0 } else { 1.0 };
        let sy = if p.y > H / 2.0 { -1.0 } else { 1.0 };
        draw_line(p.x, p.y, p.x + sx * 16.0, p.y, 2.0, MUTED);
        draw_line(p.x, p.y, p.x, p.y + sy * 16.0, 2.0, MUTED);
    }
}

pub fn preview() -> Game {
    let mut demo = Game::new(1);
    demo.player = vec2(722.0, 345.0);
    demo.angle = -0.35;
    for (position, gait) in [
        (vec2(939.0, 234.0), 0.0),
        (vec2(963.0, 290.0), 1.0),
        (vec2(899.0, 330.0), 2.0),
    ] {
        demo.zombies.push(Zombie {
            position,
            speed: 0.0,
            gait,
        });
    }
    demo
}
