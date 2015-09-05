use graphics;
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::Size;
use models::{Bullet, Wave, Enemy, Particle, Player};
use models::CollisionTestBall;
use traits::Entity;

use std::iter::Iterator;

/// A model that contains the other models and renders them
pub struct Level_0 {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub waves: Vec<Wave>,
    pub enemies: Vec<Enemy>,
    pub collision_test_balls: Vec<CollisionTestBall>,
    pub size: Size,
}

impl Level_0 {
    /// Returns a new level of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> Level_0 {
        Level_0 {
            player: Player::random(rng, size.clone()),
            particles: Vec::with_capacity(1000),
            bullets: vec![],
            waves: vec![],
            enemies: vec![],
            collision_test_balls: vec![],
            size: size
        }
    }

    /// Renders the level and everything in it
    pub fn render(&mut self, c: graphics::context::Context, g: &mut GlGraphics) {
        for particle in &self.particles {
            particle.draw(&c, g);
        }

        for bullet in &self.bullets {
            bullet.draw(&c, g);
        }

        for wave in &self.waves {
            wave.draw(&c, g);
        }

        for enemy in &self.enemies {
            enemy.draw(&c, g);
        }

        let static_renderables = self.collision_test_balls.clone();

        for (i, renderable) in &mut self.collision_test_balls.iter_mut().enumerate() {
            renderable.draw(&c, g);
        }

        let bullets_static = self.bullets.clone();
        let bullets_iter = bullets_static.iter().map(|x| x as &Entity);

        let particles_static = self.particles.clone();
        let particles_iter = particles_static.iter().map(|x| x as &Entity);

        let waves_static = self.waves.clone();
        let waves_iter = waves_static.iter().map(|x| x as &Entity);

        let enemies_static = self.enemies.clone();
        let enemies_iter = enemies_static.iter().map(|x| x as &Entity);

        let collision_test_balls_static = self.collision_test_balls.clone().into_iter();

        //let all_renderables_static = bullets_static
        //    .chain(particles_static);

        // TODO: Put this somewhere else
        // COLLISION STUFF
        for (i, renderable) in &mut self.collision_test_balls.iter_mut().enumerate() {
            renderable.update_2(4., &static_renderables, i as i64, self.player.vector.position);
        }

        self.player.draw(&c, g);
    }
}
