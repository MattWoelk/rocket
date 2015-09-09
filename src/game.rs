//! This module contains the game logic

use std::f64;
//use std::path::Path;

use graphics::{self};
use itertools;
use opengl_graphics::GlGraphics;
//use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::Key;
use rand::{self, Rng, ThreadRng};

use drawing::{color, Size};
use maths::{TAU, Point};
use models::{Player, Bullet, Wave, Enemy, Particle, Pose, Level_0};
use traits::{Advance, Collide, Position, Entity};
use models::CollisionTestBall;

use sdl2::controller::{Axis, Button};

const BULLET_RATE: f64 = 0.3;

/// The data structure that drives the game
pub struct Game {
    /// The level contains everything that needs to be drawn
    pub level: Level_0,
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub waves: Vec<Wave>,
    pub enemies: Vec<Enemy>,
    pub collision_test_balls: Vec<CollisionTestBall>,
    pub size: Size,
}

/// Additional resources needed for the game
//struct Resources {
//    font: GlyphCache<'static>
//}

impl Game {
    /// Returns a new `Game` containing a `Level_0` of the given `Size`
    pub fn new(size: Size, level: Level_0) -> Game {
        let mut rng = rand::thread_rng();
        Game {
            level: level,
            player: Player::random(&mut rng, size.clone()),
            particles: vec![],
            bullets: vec![],
            waves: vec![],
            enemies: vec![],
            collision_test_balls: vec![],
            size: size.clone(),
            //resources: Resources { font: GlyphCache::new(&Path::new("resources/FiraMono-Bold.ttf")).unwrap() }
        }
    }

    /// Processes a key press
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Left => self.level.actions.player_velocity.x = if pressed {-32768.} else {0.},
            Key::Right => self.level.actions.player_velocity.x = if pressed {32768.} else {0.},
            Key::Up => self.level.actions.player_velocity.y = if pressed {-32768.} else {0.},
            Key::Down => self.level.actions.player_velocity.y = if pressed {32768.} else {0.},
            Key::Space => self.level.actions.shoot = pressed,
            _ => ()
        }
    }

    pub fn button_press(&mut self, button: Button) {
        self.handle_button(button, true);
    }

    pub fn button_release(&mut self, button: Button) {
        self.handle_button(button, false);
    }

    fn handle_button(&mut self, button: Button, pressed: bool) {
        match button {
            Button::A => self.level.actions.grass = pressed,
            Button::B => self.level.actions.fire = pressed,
            Button::X => self.level.actions.water = pressed,
            _ => ()
        }
    }

    pub fn handle_axis(&mut self, axis: Axis, value: i32) {
        // TODO: set the dead zone based on the magnitude instead of the single axis value
        // will require handling both axes at once.
        let dead_zoned_value = if value.abs() < 5000 {0} else {value - (5000 * value.signum())};

        match axis {
            Axis::LeftX => self.level.actions.player_velocity.x = dead_zoned_value as f64,
            Axis::LeftY => self.level.actions.player_velocity.y = dead_zoned_value as f64,
            _ => ()
        }
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update(&mut self, dt: f64) {
        self.level.timers.current_time += dt;

        let displacement = dt * self.level.actions.player_velocity / 32000.0 * 400.0;

        self.player.advance_with_wrapping(displacement, self.size.clone());

        // Update particles
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // Remove old particles
        self.particles.retain(|p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if self.level.timers.current_time - self.level.timers.last_tail_particle > 0.05 {
            self.level.timers.last_tail_particle = self.level.timers.current_time;
            self.particles.push(Particle::new(self.player.vector.clone().invert(), 0.5));
        }

        // Add bullets
        if self.level.actions.shoot && self.level.timers.current_time - self.level.timers.last_shoot > BULLET_RATE {
            self.level.timers.last_shoot = self.level.timers.current_time;
            self.waves.push(Wave::new(self.player.position().clone()));
        }

        if self.level.actions.grass && self.level.timers.current_time - self.level.timers.last_shoot > BULLET_RATE {
            self.level.timers.last_shoot = self.level.timers.current_time;
            self.waves.push(Wave::new_grass(self.player.position().clone()));
        }

        if self.level.actions.fire && self.level.timers.current_time - self.level.timers.last_shoot > BULLET_RATE {
            self.level.timers.last_shoot = self.level.timers.current_time;
            self.waves.push(Wave::new_fire(self.player.position().clone()));
        }

        if self.level.actions.water && self.level.timers.current_time - self.level.timers.last_shoot > BULLET_RATE {
            self.level.timers.last_shoot = self.level.timers.current_time;
            self.waves.push(Wave::new_water(self.player.position().clone()));
        }

        for wave in &mut self.waves {
            wave.update(dt);
        }

        { // Shorten the lifetime of size
            let size = &self.size;
            self.waves.retain(|w| w.radius < (size.width + size.height) * 0.75);
        }

        // Spawn enemies at random locations
        if self.level.timers.current_time - self.level.timers.last_spawned_enemy > 1.0 {
            self.level.timers.last_spawned_enemy = self.level.timers.current_time;
            let mut new_enemy: Enemy;
            loop {
                new_enemy = Enemy::new(Pose::random(&mut self.level.rng, self.size.clone()));
                if !self.player.collides_with(&new_enemy) {
                    break;
                }
            }
            self.enemies.push(new_enemy);
        }

        // Move enemies in the player's direction
        for enemy in &mut self.enemies {
            enemy.update(dt * 100.0, self.player.position());
        }

        self.handle_player_collisions();
    }

    // TODO: to be removed
    fn handle_bullet_collisions(&mut self) {
        let old_enemy_count = self.enemies.len();

        {
            // We introduce a scope to shorten the lifetime of the borrows below
            // The references are to avoid using self in the closure
            // (the borrow checker doesn't like that)
            let bullets = &mut self.bullets;
            let enemies = &mut self.enemies;
            let particles = &mut self.particles;

            bullets.retain(|bullet| {
                // Remove the first enemy that collides with a bullet (if any)
                // Add an explosion on its place
                if let Some((index, position)) = enemies.iter().enumerate()
                    .find(|&(_, enemy)| enemy.collides_with(bullet))
                    .map(|(index, enemy)| (index, enemy.position()))
                {
                    Game::make_explosion(particles, position, 10);
                    enemies.remove(index);
                    false
                } else {
                    true
                }
            });
        }

        let killed_enemies = (old_enemy_count - self.enemies.len()) as u32;
        self.level.score += 10 * killed_enemies;
    }

    /// reset our game-state
    fn reset(&mut self) {
        // Reset player position
        *self.player.x_mut() = self.size.random_x(&mut self.level.rng);
        *self.player.y_mut() = self.size.random_y(&mut self.level.rng);

        // Reset score
        self.level.score = 0;

        // Remove all enemies
        self.enemies.clear();
    }

    /// Handles collisions between the player and the enemies
    fn handle_player_collisions(&mut self) {
        if self.enemies.iter().any(|enemy| self.player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = self.player.position();
            Game::make_explosion(&mut self.particles, ppos, 8);

            self.reset();
        }
    }

    // Generates a new explosion of the given intensity at the given position. This works best with values between 5 and 25
    fn make_explosion(particles: &mut Vec<Particle>, position: Point, intensity: u8) {
        for rotation in itertools::linspace(0.0, TAU, 30) {
            for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
                particles.push(Particle::new(Pose::new(position.clone(), rotation), ttl));
            }
        }
    }

    #[allow(unused_variables)]
    pub fn spawn_circle_with_collision_colouring(&mut self, position: Point) {
        let mut entity = CollisionTestBall::new();
        entity.velocity = Point::new(1., 1.);
        self.collision_test_balls.push(entity);
        // TODO
        //let player_position = self.player.position();
        //let circle = Circle {
        //    radius: 5.,
        //    centre: position,
        //};

        // TODO: This rendering needs to be in another place.
        //let colour = if circle.collide_with_point(player_position) {
        //    color::BLUE
        //} else {
        //    color::RED
        //};

        //Ellipse::new(colour).draw(
        //    [circle.centre.x - circle.radius, circle.centre.y - circle.radius, 0., 0.],
        //    &c.draw_state, c.transform, gl);
    }

    /// Renders the level and everything in it
    pub fn render(&mut self, c: graphics::context::Context, g: &mut GlGraphics) {
        graphics::clear(color::BLACK, g);

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

        // Render the score
        let mut text = graphics::Text::new(22);
        text.color = color::ORANGE;
        //text.draw(&format!("Score: {}", self.level.score),
        //          &mut self.resources.font,
        //          &c.draw_state,
        //          c.trans(10.0, 20.0).transform,
        //          g);
    }
}
