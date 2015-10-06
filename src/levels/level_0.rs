use drawing::Size;
use models::{Wave, Entity, Particle, Player, Pose};
use traits::{Advance, Position, Collide, Level};
use rand::{self, ThreadRng};
use game::{Game, BULLET_RATE};
use levels::{Controls, Actions, Timers};

use std::iter::Iterator;

/// A model that contains the other models and renders them
#[derive(Clone)]
pub struct Level0 {
    pub score: u32,
    pub actions: Actions,
    pub timers: Timers,
    pub rng: ThreadRng,
}

impl Level0 {
    /// Returns a new level of the given size
    pub fn new() -> Level0 {
        let rng = rand::thread_rng();

        Level0 {
            score: 0,
            actions: Actions::default(),
            timers: Timers::new(),
            rng: rng,
        }
    }
}

impl Level for Level0 {
    fn handle_control(&mut self, control: Controls) {
        match control {
            Controls::X1(val) => self.actions.player_velocity.x = val as f64,
            Controls::Y1(val) => self.actions.player_velocity.y = val as f64,
            Controls::X(pressed) => self.actions.shoot = pressed,
            _ => ()
        }
    }

    #[allow(unused_variables)]
    fn reset(&mut self,
             //particles: &mut Vec<Particle>,
             player: &mut Player,
             //waves: &mut Vec<Wave>,
             entities: &mut Vec<Entity>,
             size: &Size,
             dt: f64) {
        // Reset player position
        *player.x_mut() = size.random_x(&mut self.rng);
        *player.y_mut() = size.random_y(&mut self.rng);

        // Reset score
        self.score = 0;

        // Remove all enemies
        entities.clear();
    }

    fn update(&mut self,
              //particles: &mut Vec<Particle>,
              player: &mut Player,
              //waves: &mut Vec<Wave>,
              entities: &mut Vec<Entity>,
              size: &Size,
              dt: f64) {
        self.timers.current_time += dt;

        let displacement = dt * self.actions.player_velocity / 32000.0 * 400.0;

        player.advance_with_wrapping(displacement, size.clone());

        // Update particles
        //for particle in particles.iter_mut() {
        //    particle.update(dt);
        //}

        // Remove old particles
        //particles.retain(|p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        //if self.timers.current_time - self.timers.last_tail_particle > 0.05 {
        //    self.timers.last_tail_particle = self.timers.current_time;
        //    particles.push(Particle::new(player.vector.clone().invert(), 0.5));
        //}

        // Add waves
        //if self.actions.shoot && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
        //    self.timers.last_shoot = self.timers.current_time;
        //    waves.push(Wave::new(player.position().clone()));
        //}

        //if self.actions.grass && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
        //    self.timers.last_shoot = self.timers.current_time;
        //    waves.push(Wave::new_grass(player.position().clone()));
        //}

        //if self.actions.fire && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
        //    self.timers.last_shoot = self.timers.current_time;
        //    waves.push(Wave::new_fire(player.position().clone()));
        //}

        //if self.actions.water && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
        //    self.timers.last_shoot = self.timers.current_time;
        //    waves.push(Wave::new_water(player.position().clone()));
        //}

        //for wave in waves.iter_mut() {
        //    wave.update(dt);
        //}

        //waves.retain(|w| w.radius < (size.width + size.height) * 0.75);

        // Spawn enemies at random locations
        //if self.timers.current_time - self.timers.last_spawned_enemy > 1.0 {
        //    self.timers.last_spawned_enemy = self.timers.current_time;
        //    let mut new_enemy: Enemy;
        //    loop {
        //        new_enemy = Enemy::new(Pose::random(&mut self.rng, size.clone()));
        //        if !player.collides_with(&new_enemy) {
        //            break;
        //        }
        //    }
        //    enemies.push(new_enemy);
        //}

        // Move enemies in the player's direction
        for entity in entities.iter_mut() {
            entity.update(dt * 100.0);
        }

        //handle player collisions
        //if enemies.iter().any(|enemy| player.collides_with(enemy)) {
        //    // Make an explosion where the player was
        //    let ppos = player.position();
        //    Game::make_explosion(particles, ppos, 8);

        //    self.reset(
        //        particles,
        //        player,
        //        waves,
        //        enemies,
        //        size,
        //        dt);
        //}
    }
}
