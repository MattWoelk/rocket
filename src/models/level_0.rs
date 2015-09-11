use graphics;
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::Size;
use models::{Bullet, Wave, Enemy, Particle, Player, Pose};
use models::CollisionTestBall;
use traits::{Advance, Position, Collide, Entity, Level};
use rand::{self, ThreadRng};
use maths::Point;
use game::{Game, BULLET_RATE};

use std::iter::Iterator;

/// A model that contains the other models and renders them
pub struct Level_0 {
    pub score: u32,
    pub actions: Actions,
    pub timers: Timers,
    pub rng: ThreadRng,
}

pub enum Controls {
    A(bool),
    B(bool),
    X(bool),
    //Y(bool),
    //LT(bool),
    //RT(bool),
    //LB(bool),
    //RB(bool),
    X1(i64),
    Y1(i64),
    //X2(i64),
    //Y2(i64),
    None,
}

#[derive(Default)]
pub struct Actions {
    pub player_velocity: Point,
    pub boost: bool,
    pub shoot: bool,
    pub grass: bool,
    pub water: bool,
    pub fire: bool,
}

/// Timers to handle creation of enemies and particles
pub struct Timers {
    pub current_time: f64,
    pub last_tail_particle: f64,
    pub last_shoot: f64,
    pub last_spawned_enemy: f64
}

impl Timers {
    fn new() -> Self {
        Timers {
            current_time: 0.,
            last_tail_particle: 0.,
            last_shoot: 0.,
            last_spawned_enemy: 0.,
        }
    }
}

impl Level_0 {
    /// Returns a new level of the given size
    pub fn new() -> Level_0 {
        let mut rng = rand::thread_rng();

        Level_0 {
            score: 0,
            actions: Actions::default(),
            timers: Timers::new(),
            rng: rng,
        }
    }

    /// reset our game-state
    fn reset(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>, size: &Size) {
        // Reset player position
        *player.x_mut() = size.random_x(&mut self.rng);
        *player.y_mut() = size.random_y(&mut self.rng);

        // Reset score
        self.score = 0;

        // Remove all enemies
        enemies.clear();
    }
}

impl Level for Level_0 {
    fn handle_control(&mut self, control: Controls) {
        match control {
            Controls::X1(val) => self.actions.player_velocity.x = val as f64,
            Controls::Y1(val) => self.actions.player_velocity.y = val as f64,
            Controls::X(pressed) => self.actions.shoot = pressed,
            _ => ()
        }
    }

    fn update(&mut self,
              particles: &mut Vec<Particle>,
              player: &mut Player,
              waves: &mut Vec<Wave>,
              enemies: &mut Vec<Enemy>,
              size: &Size,
              dt: f64) {
        self.timers.current_time += dt;

        let displacement = dt * self.actions.player_velocity / 32000.0 * 400.0;

        player.advance_with_wrapping(displacement, size.clone());

        // Update particles
        for particle in particles.iter_mut() {
            particle.update(dt);
        }

        // Remove old particles
        particles.retain(|p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if self.timers.current_time - self.timers.last_tail_particle > 0.05 {
            self.timers.last_tail_particle = self.timers.current_time;
            particles.push(Particle::new(player.vector.clone().invert(), 0.5));
        }

        // Add waves
        if self.actions.shoot && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            waves.push(Wave::new(player.position().clone()));
        }

        if self.actions.grass && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            waves.push(Wave::new_grass(player.position().clone()));
        }

        if self.actions.fire && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            waves.push(Wave::new_fire(player.position().clone()));
        }

        if self.actions.water && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            waves.push(Wave::new_water(player.position().clone()));
        }

        for wave in waves.iter_mut() {
            wave.update(dt);
        }

        waves.retain(|w| w.radius < (size.width + size.height) * 0.75);

        // Spawn enemies at random locations
        if self.timers.current_time - self.timers.last_spawned_enemy > 1.0 {
            self.timers.last_spawned_enemy = self.timers.current_time;
            let mut new_enemy: Enemy;
            loop {
                new_enemy = Enemy::new(Pose::random(&mut self.rng, size.clone()));
                if !player.collides_with(&new_enemy) {
                    break;
                }
            }
            enemies.push(new_enemy);
        }

        // Move enemies in the player's direction
        for enemy in enemies.iter_mut() {
            enemy.update(dt * 100.0, player.position());
        }

        //handle player collisions
        if enemies.iter().any(|enemy| player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = player.position();
            Game::make_explosion(particles, ppos, 8);

            self.reset(player, enemies, size);
        }
    }
}
