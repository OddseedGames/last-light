use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use oddseed_last_light::game::Game;

pub struct Audio {
    pub enabled: bool,
    shotgun: Sound,
    reload: Sound,
    hurt: Sound,
}

impl Audio {
    pub async fn load() -> Self {
        Self {
            enabled: false,
            shotgun: load_sound_from_bytes(include_bytes!("../assets/shotgun.wav"))
                .await
                .unwrap(),
            reload: load_sound_from_bytes(include_bytes!("../assets/reload.wav"))
                .await
                .unwrap(),
            hurt: load_sound_from_bytes(include_bytes!("../assets/hurt.wav"))
                .await
                .unwrap(),
        }
    }

    pub fn play_changes(&self, game: &Game, before: (u32, u8, f32)) {
        if !self.enabled {
            return;
        }
        if game.fired > before.0 {
            play(&self.shotgun, 0.55);
        }
        if game.health < before.1 {
            play(&self.hurt, 0.45);
        }
        if game.reload_left > before.2 {
            play(&self.reload, 0.40);
        }
    }
}

fn play(sound: &Sound, volume: f32) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume,
        },
    );
}
