use super::theme::*;
use macroquad::prelude::*;
use oddseed_last_light::{game::*, layout::*, lighting::*};

pub fn draw(game: &Game, font: &Font, point: Vec2) {
    if game.phase == Phase::Playing {
        return;
    }
    if game.phase == Phase::Ready {
        draw_rectangle(
            29.0,
            111.0,
            530.0,
            HEIGHT - 2.0,
            alpha(rgb(8, 15, 14), 0.96),
        );
        txt(
            font,
            "НОЧНОЕ ВЫЖИВАНИЕ / ВИД СВЕРХУ",
            88.0,
            224.0,
            11.0,
            AMBER,
        );
        txt(font, "ТЕМНОТА", 84.0, 302.0, 64.0, INK);
        txt(font, "ГОЛОДНА.", 84.0, 369.0, 64.0, INK);
        txt(
            font,
            "Держи их в луче. Не стой на месте.",
            88.0,
            420.0,
            15.0,
            INK,
        );
        txt(font, "За спиной ничего не видно.", 88.0, 457.0, 13.0, MUTED);
        txt(
            font,
            "Один выстрел. Один мёртвый зомби.",
            88.0,
            481.0,
            13.0,
            MUTED,
        );
        button(font, "ВОЙТИ В ТЕМНОТУ       ENTER", START_BUTTON, point);
        txt(
            font,
            "6 патронов. 3 жизни. Сколько продержишься?",
            88.0,
            622.0,
            12.0,
            MUTED,
        );
        draw_line(88.0, 653.0, 488.0, 653.0, 1.0, LINE);
        txt(
            font,
            "Фонарь и ствол смотрят в одну сторону.",
            88.0,
            683.0,
            11.0,
            MUTED,
        );
        txt(
            font,
            "Попасть можно только в освещённую цель.",
            88.0,
            705.0,
            11.0,
            MUTED,
        );
    } else {
        draw_rectangle(
            29.0,
            111.0,
            WIDTH - 2.0,
            HEIGHT - 2.0,
            alpha(rgb(5, 10, 9), 0.88),
        );
        let card = Rect::new(359.0, 250.0, 562.0, 370.0);
        panel(card, rgb(14, 23, 20));
        let dead = game.phase == Phase::Dead;
        txt(
            font,
            if dead {
                "СВЕТ ПОГАС"
            } else {
                "ТИШИНА."
            },
            399.0,
            327.0,
            42.0,
            if dead { DANGER } else { INK },
        );
        txt(
            font,
            if dead {
                "Они подобрались слишком близко."
            } else {
                "Ночь подождёт. Переведи дыхание."
            },
            399.0,
            370.0,
            14.0,
            MUTED,
        );
        txt(
            font,
            &format!(
                "ВРЕМЯ  {}     УБИТО  {}",
                time_label(game.elapsed),
                game.kills
            ),
            399.0,
            425.0,
            17.0,
            INK,
        );
        button(
            font,
            if dead {
                "ЕЩЁ ОДНА НОЧЬ         ENTER"
            } else {
                "ПРОДОЛЖИТЬ            ENTER"
            },
            RESUME_BUTTON,
            point,
        );
        txt(
            font,
            if dead {
                "Наводи свет мышью. Проверяй, что за спиной."
            } else {
                "N  начать заново       ESC  вернуться"
            },
            399.0,
            560.0,
            12.0,
            MUTED,
        );
    }
}

pub fn reticle(game: &Game, point: Vec2) {
    if game.phase != Phase::Playing || !arena_rect().contains(point) {
        return;
    }
    let lit = in_beam(game.player, game.angle, point - ARENA);
    let color = alpha(if lit { AMBER } else { MUTED }, 0.85);
    draw_circle_lines(point.x, point.y, 8.0, 1.0, color);
    for direction in [
        vec2(1.0, 0.0),
        vec2(-1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(0.0, -1.0),
    ] {
        let start = point + direction * 12.0;
        let end = point + direction * 17.0;
        draw_line(start.x, start.y, end.x, end.y, 1.0, color);
    }
}
