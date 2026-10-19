use super::theme::*;
use macroquad::prelude::*;
use oddseed_last_light::{game::*, layout::*};

pub fn draw(game: &Game, font: &Font, sound: bool, best: u32, point: Vec2) {
    txt(font, "ODDSEED / 002", 28.0, 32.0, 11.0, AMBER);
    txt(font, "ПОСЛЕДНИЙ ЛУЧ", 28.0, 73.0, 29.0, INK);
    txt(font, "ВРЕМЯ", 530.0, 32.0, 10.0, MUTED);
    txt(font, &time_label(game.elapsed), 530.0, 72.0, 30.0, INK);
    txt(font, "УБИТО", 680.0, 32.0, 10.0, MUTED);
    txt(
        font,
        &format!("{:03}", game.kills),
        680.0,
        72.0,
        30.0,
        AMBER,
    );
    txt(font, "ЗДОРОВЬЕ", 820.0, 32.0, 10.0, MUTED);
    for i in 0..3 {
        let x = 820.0 + i as f32 * 26.0;
        let color = if i < game.health { INK } else { LINE };
        draw_rectangle(x, 54.0, 18.0, 8.0, color);
        draw_rectangle(x + 5.0, 49.0, 8.0, 18.0, color);
    }
    for (rect, label) in [
        (
            SOUND_BUTTON,
            if sound {
                "ЗВУК ВКЛ"
            } else {
                "ЗВУК ВЫКЛ"
            },
        ),
        (PAUSE_BUTTON, "ПАУЗА"),
    ] {
        panel(
            rect,
            if rect.contains(point) {
                rgb(31, 41, 37)
            } else {
                rgb(18, 25, 23)
            },
        );
        txt(font, label, rect.x + 12.0, rect.y + 23.0, 12.0, MUTED);
    }
    txt(
        font,
        "ПЕРИМЕТР 07 / ЗОНА ОТЧУЖДЕНИЯ",
        45.0,
        135.0,
        10.0,
        alpha(MUTED, 0.65),
    );
    right(
        font,
        "НОЧЬ  /  02:17",
        1235.0,
        135.0,
        10.0,
        alpha(MUTED, 0.65),
    );
    txt(font, "ПОМПОВЫЙ ДРОБОВИК", 28.0, 802.0, 11.0, MUTED);
    for i in 0..MAGAZINE {
        let x = 29.0 + i as f32 * 18.0;
        let color = if i < game.ammo { AMBER } else { LINE };
        draw_rectangle(x, 818.0, 10.0, 22.0, color);
        draw_rectangle(x, 835.0, 10.0, 3.0, rgb(132, 117, 77));
    }
    txt(font, &format!("{} / 6", game.ammo), 156.0, 836.0, 23.0, INK);
    let reload = if game.reload_left > 0.0 {
        "ПЕРЕЗАРЯДКА..."
    } else {
        "R  ПЕРЕЗАРЯДИТЬ"
    };
    txt(
        font,
        reload,
        274.0,
        832.0,
        11.0,
        if game.reload_left > 0.0 { AMBER } else { MUTED },
    );
    txt(
        font,
        "WASD  идти    МЫШЬ  свет    ЛКМ  стрелять",
        540.0,
        808.0,
        12.0,
        INK,
    );
    txt(
        font,
        "Q / E  вращать свет    ESC  пауза    M  звук",
        540.0,
        836.0,
        11.0,
        MUTED,
    );
    right(
        font,
        &format!("РЕКОРД  {:03}", best.max(game.kills)),
        1252.0,
        832.0,
        12.0,
        AMBER,
    );
    draw_line(28.0, 858.0, 1252.0, 858.0, 1.0, LINE);
    txt(
        font,
        "ОНИ ВИДЯТ ТЕБЯ. ТЫ ВИДИШЬ ТОЛЬКО СВЕТ.",
        28.0,
        877.0,
        9.0,
        MUTED,
    );
    right(font, "RUST / MACROQUAD", 1252.0, 877.0, 9.0, MUTED);
}
