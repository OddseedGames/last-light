use crate::lighting::{forward, in_beam, BEAM_CENTER, BEAM_RX};
use macroquad::math::{vec2, Vec2};
use std::f32::consts::{PI, TAU};

pub const WIDTH: f32 = 1224.0;
pub const HEIGHT: f32 = 660.0;
pub const PLAYER_RADIUS: f32 = 14.0;
pub const ZOMBIE_RADIUS: f32 = 15.0;
pub const PLAYER_SPEED: f32 = 190.0;
pub const SHOT_SPREAD: f32 = 0.27;
pub const SHOT_DELAY: f32 = 0.52;
pub const RELOAD_TIME: f32 = 1.65;
pub const MAGAZINE: u8 = 6;
pub const PELLETS: usize = 11;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Ready,
    Playing,
    Paused,
    Dead,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    pub movement: Vec2,
    pub aim: Option<f32>,
    pub turn: f32,
    pub fire: bool,
    pub reload: bool,
}

#[derive(Clone, Debug)]
pub struct Zombie {
    pub position: Vec2,
    pub speed: f32,
    pub gait: f32,
}

#[derive(Clone, Debug)]
pub struct Shot {
    pub origin: Vec2,
    pub ends: Vec<Vec2>,
    pub age: f32,
}

#[derive(Clone, Debug)]
pub struct Remains {
    pub position: Vec2,
    pub angle: f32,
    pub age: f32,
}

pub struct Game {
    pub phase: Phase,
    pub player: Vec2,
    pub angle: f32,
    pub health: u8,
    pub ammo: u8,
    pub reload_left: f32,
    pub cooldown: f32,
    pub invulnerable: f32,
    pub elapsed: f32,
    pub kills: u32,
    pub fired: u32,
    pub zombies: Vec<Zombie>,
    pub shots: Vec<Shot>,
    pub remains: Vec<Remains>,
    spawn_left: f32,
    rng: u64,
}

impl Game {
    pub fn new(seed: u64) -> Self {
        Self {
            phase: Phase::Ready,
            player: vec2(WIDTH / 2.0, HEIGHT / 2.0),
            angle: -0.45,
            health: 3,
            ammo: MAGAZINE,
            reload_left: 0.0,
            cooldown: 0.0,
            invulnerable: 0.0,
            elapsed: 0.0,
            kills: 0,
            fired: 0,
            zombies: Vec::new(),
            shots: Vec::new(),
            remains: Vec::new(),
            spawn_left: 0.4,
            rng: seed.max(1),
        }
    }

    pub fn start(&mut self) {
        let seed = self.rng;
        *self = Self::new(seed);
        self.phase = Phase::Playing;
    }

    pub fn toggle_pause(&mut self) {
        self.phase = match self.phase {
            Phase::Playing => Phase::Paused,
            Phase::Paused => Phase::Playing,
            other => other,
        };
    }

    pub fn visible(&self, zombie: &Zombie) -> bool {
        in_beam(self.player, self.angle, zombie.position)
    }

    pub fn reload(&mut self) {
        if self.phase == Phase::Playing && self.ammo < MAGAZINE && self.reload_left <= 0.0 {
            self.reload_left = RELOAD_TIME;
        }
    }

    fn random(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng >> 40) as f32 / (1u32 << 24) as f32
    }

    pub fn spawn(&mut self) {
        let side = (self.random() * 4.0) as u8;
        let along = self.random();
        let position = match side {
            0 => vec2(along * WIDTH, -40.0),
            1 => vec2(WIDTH + 40.0, along * HEIGHT),
            2 => vec2(along * WIDTH, HEIGHT + 40.0),
            _ => vec2(-40.0, along * HEIGHT),
        };
        let speed = 45.0 + self.random() * 22.0 + (self.elapsed * 0.18).min(28.0);
        let gait = self.random() * TAU;
        self.zombies.push(Zombie {
            position,
            speed,
            gait,
        });
    }

    pub fn fire(&mut self) -> bool {
        if self.phase != Phase::Playing || self.cooldown > 0.0 || self.reload_left > 0.0 {
            return false;
        }
        if self.ammo == 0 {
            self.reload();
            return false;
        }
        self.ammo -= 1;
        self.fired += 1;
        self.cooldown = SHOT_DELAY;
        let origin = self.player;
        let mut ends = Vec::with_capacity(PELLETS);
        let mut dead = vec![false; self.zombies.len()];
        for pellet in 0..PELLETS {
            let angle =
                self.angle + (pellet as f32 / (PELLETS - 1) as f32 * 2.0 - 1.0) * SHOT_SPREAD;
            let direction = forward(angle);
            let mut nearest = BEAM_CENTER + BEAM_RX;
            let mut target = None;
            for (index, zombie) in self.zombies.iter().enumerate() {
                if dead[index] || !self.visible(zombie) {
                    continue;
                }
                let relative = zombie.position - origin;
                let along = relative.dot(direction);
                let across = relative.perp_dot(direction).abs();
                if across < ZOMBIE_RADIUS && along > 0.0 {
                    let hit = along - (ZOMBIE_RADIUS.powi(2) - across.powi(2)).sqrt();
                    if hit < nearest {
                        nearest = hit.max(0.0);
                        target = Some(index);
                    }
                }
            }
            if let Some(index) = target {
                dead[index] = true;
                self.kills += 1;
                self.remains.push(Remains {
                    position: self.zombies[index].position,
                    angle,
                    age: 0.0,
                });
            }
            ends.push(origin + direction * nearest);
        }
        let mut index = 0;
        self.zombies.retain(|_| {
            let alive = !dead[index];
            index += 1;
            alive
        });
        self.shots.push(Shot {
            origin,
            ends,
            age: 0.0,
        });
        if self.ammo == 0 {
            self.reload();
        }
        true
    }

    pub fn update(&mut self, dt: f32, input: Input) {
        if self.phase != Phase::Playing || !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let dt = dt.min(0.05);
        self.elapsed += dt;
        self.cooldown = (self.cooldown - dt).max(0.0);
        self.invulnerable = (self.invulnerable - dt).max(0.0);
        if self.reload_left > 0.0 {
            self.reload_left = (self.reload_left - dt).max(0.0);
            if self.reload_left == 0.0 {
                self.ammo = MAGAZINE;
            }
        }
        if let Some(angle) = input.aim {
            self.angle = angle;
        }
        self.angle = (self.angle + input.turn * 3.5 * dt + PI).rem_euclid(TAU) - PI;
        self.player += input.movement.clamp_length_max(1.0) * PLAYER_SPEED * dt;
        self.player.x = self.player.x.clamp(24.0, WIDTH - 24.0);
        self.player.y = self.player.y.clamp(24.0, HEIGHT - 24.0);
        if input.reload {
            self.reload();
        }
        for shot in &mut self.shots {
            shot.age += dt;
        }
        self.shots.retain(|shot| shot.age < 0.16);
        for remains in &mut self.remains {
            remains.age += dt;
        }
        self.remains.retain(|remains| remains.age < 8.0);
        if input.fire {
            self.fire();
        }
        self.spawn_left -= dt;
        while self.spawn_left <= 0.0 {
            self.spawn();
            self.spawn_left += (1.8 - self.elapsed * 0.011).max(0.38);
        }
        let positions: Vec<Vec2> = self.zombies.iter().map(|zombie| zombie.position).collect();
        for (index, zombie) in self.zombies.iter_mut().enumerate() {
            let chase = (self.player - zombie.position).normalize_or_zero();
            let mut separation = Vec2::ZERO;
            for (other, position) in positions.iter().enumerate() {
                let delta = zombie.position - *position;
                let distance = delta.length();
                if other != index && distance > 0.01 && distance < ZOMBIE_RADIUS * 2.2 {
                    separation += delta / distance * (1.0 - distance / (ZOMBIE_RADIUS * 2.2));
                }
            }
            zombie.position += (chase + separation * 1.8).clamp_length_max(1.0) * zombie.speed * dt;
            zombie.gait += dt * zombie.speed * 0.10;
            let delta = zombie.position - self.player;
            if delta.length() < PLAYER_RADIUS + ZOMBIE_RADIUS {
                if self.invulnerable <= 0.0 {
                    self.health -= 1;
                    self.invulnerable = 1.5;
                    if self.health == 0 {
                        self.phase = Phase::Dead;
                    }
                }
                zombie.position =
                    self.player + delta.try_normalize().unwrap_or(vec2(1.0, 0.0)) * 68.0;
            }
        }
    }
}
