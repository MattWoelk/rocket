use graphics;
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::Size;
use models::{Bullet, Wave, Enemy, Particle, Player};
use models::Entity;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub waves: Vec<Wave>,
    pub enemies: Vec<Enemy>,
    pub renderables: Vec<Entity>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        World {
            player: Player::random(rng, size.clone()),
            particles: Vec::with_capacity(1000),
            bullets: vec![],
            waves: vec![],
            enemies: vec![],
            renderables: vec![],
            size: size
        }
    }

    /// Renders the world and everything in it
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

        let static_renderables = self.renderables.clone();

        for (i, renderable) in &mut self.renderables.iter_mut().enumerate() {
            renderable.draw(&c, g);
            renderable.update_2(4., &static_renderables, i as i64, self.player.vector.position);
        }

        self.player.draw(&c, g);
    }
}
