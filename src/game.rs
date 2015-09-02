//! This module contains the game logic

use std::f64;
//use std::path::Path;

use graphics::{self};
use itertools;
use opengl_graphics::GlGraphics;
//use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::Key;
use rand::{self, Rng, ThreadRng};

use drawing::{color, Point, Size};
use models::{Bullet, Wave, Enemy, Particle, Pose, World};
use traits::{Advance, Collide, Position};
use models::Entity;

use sdl2::controller::{Axis, Button};

const BULLET_RATE: f64 = 0.3;

/// The data structure that drives the game
pub struct Game {
    /// The world contains everything that needs to be drawn
    world: World,
    /// The current score of the player
    score: u32,
    /// The active actions
    actions: Actions,
    /// Timers needed by the game
    timers: Timers,
    /// Resources needed for drawing
    //resources: Resources
    /// A random number generator
    rng: ThreadRng,
}

/// Active actions (toggled by user input)
#[derive(Default)]
struct Actions {
    player_velocity: Point,
    boost: bool,
    shoot: bool,
    grass: bool,
    water: bool,
    fire: bool,
}

/// Timers to handle creation of bullets, enemies and particles
struct Timers {
    current_time: f64,
    last_tail_particle: f64,
    last_shoot: f64,
    last_spawned_enemy: f64
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

/// Additional resources needed for the game
//struct Resources {
//    font: GlyphCache<'static>
//}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size`
    pub fn new(size: Size) -> Game {
        let mut rng = rand::thread_rng();
        Game {
            world: World::new(&mut rng, size),
            score: 0,
            actions: Actions::default(),
            timers: Timers::new(),
            rng: rng,
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
            Key::Left => self.actions.player_velocity.x = if pressed {-32768.} else {0.},
            Key::Right => self.actions.player_velocity.x = if pressed {32768.} else {0.},
            Key::Up => self.actions.player_velocity.y = if pressed {-32768.} else {0.},
            Key::Down => self.actions.player_velocity.y = if pressed {32768.} else {0.},
            Key::Space => self.actions.shoot = pressed,
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
            Button::A => self.actions.grass = pressed,
            Button::B => self.actions.fire = pressed,
            Button::X => self.actions.water = pressed,
            _ => ()
        }
    }

    pub fn handle_axis(&mut self, axis: Axis, value: i32) {
        // TODO: set the dead zone based on the magnitude instead of the single axis value
        // will require handling both axes at once.
        let dead_zoned_value = if value.abs() < 5000 {0} else {value - (5000 * value.signum())};

        match axis {
            Axis::LeftX => self.actions.player_velocity.x = dead_zoned_value as f64,
            Axis::LeftY => self.actions.player_velocity.y = dead_zoned_value as f64,
            _ => ()
        }
    }

    /// Renders the game to the screen
    pub fn render(&mut self, c: graphics::context::Context, g: &mut GlGraphics) {
        // Clear everything
        graphics::clear(color::BLACK, g);

        // Render the world
        self.world.render(c, g);

        // Render the score
        let mut text = graphics::Text::new(22);
        text.color = color::ORANGE;
        //text.draw(&format!("Score: {}", self.score),
        //          &mut self.resources.font,
        //          &c.draw_state,
        //          c.trans(10.0, 20.0).transform,
        //          g);
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;

        let displacement = dt * self.actions.player_velocity / 32000.0 * 400.0;

        self.world.player.advance_with_wrapping(displacement, self.world.size.clone());

        // Update particles
        for particle in &mut self.world.particles {
            particle.update(dt);
        }

        // Remove old particles
        self.world.particles.retain(|p| p.ttl > 0.0);

        // Add new particles at the player's position, to leave a trail
        if self.timers.current_time - self.timers.last_tail_particle > 0.05 {
            self.timers.last_tail_particle = self.timers.current_time;
            self.world.particles.push(Particle::new(self.world.player.vector.clone().invert(), 0.5));
        }

        // Add bullets
        if self.actions.shoot && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            let bullet_angle = if self.actions.boost {self.rng.gen::<f64>() - 0.5} else {0.};
            self.world.bullets.push(Bullet::new(Pose::new(self.world.player.nose(), self.world.player.angle_radians() + bullet_angle)));
            self.world.waves.push(Wave::new(self.world.player.position().clone()));
        }

        if self.actions.grass && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            self.world.waves.push(Wave::new_grass(self.world.player.position().clone()));
        }

        if self.actions.fire && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            self.world.waves.push(Wave::new_fire(self.world.player.position().clone()));
        }

        if self.actions.water && self.timers.current_time - self.timers.last_shoot > BULLET_RATE {
            self.timers.last_shoot = self.timers.current_time;
            self.world.waves.push(Wave::new_water(self.world.player.position().clone()));
        }

        // Advance bullets
        for bullet in &mut self.world.bullets {
            bullet.update(dt * 500.0);
        }

        for wave in &mut self.world.waves {
            wave.update(dt);
        }

        // Remove bullets outside the viewport
        { // Shorten the lifetime of size
        let size = &self.world.size;
        self.world.bullets.retain(|b| size.contains(b.position()));
        self.world.waves.retain(|w| w.radius < (size.width + size.height) * 0.75);
        }

        // Spawn enemies at random locations
        if self.timers.current_time - self.timers.last_spawned_enemy > 1.0 {
            self.timers.last_spawned_enemy = self.timers.current_time;
            let mut new_enemy: Enemy;
            loop {
                new_enemy = Enemy::new(Pose::random(&mut self.rng, self.world.size.clone()));
                if !self.world.player.collides_with(&new_enemy) {
                    break;
                }
            }
            self.world.enemies.push(new_enemy);
        }

        // Move enemies in the player's direction
        for enemy in &mut self.world.enemies {
            enemy.update(dt * 100.0, self.world.player.position());
        }

        self.handle_player_collisions();
        self.handle_bullet_collisions();
    }

    /// Handles collisions between the bullets and the enemies
    ///
    /// When an enemy is reached by a bullet, both the enemy and the bullet
    /// will be removed. Additionally, the score will be increased by 10
    fn handle_bullet_collisions(&mut self) {
        let old_enemy_count = self.world.enemies.len();

        { // We introduce a scope to shorten the lifetime of the borrows below
        // The references are to avoid using self in the closure
        // (the borrow checker doesn't like that)
        let bullets = &mut self.world.bullets;
        let enemies = &mut self.world.enemies;
        let particles = &mut self.world.particles;

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

        let killed_enemies = (old_enemy_count - self.world.enemies.len()) as u32;
        self.score += 10 * killed_enemies;
    }

    /// reset our game-state
    fn reset(&mut self) {
        // Reset player position
        *self.world.player.x_mut() = self.world.size.random_x(&mut self.rng);
        *self.world.player.y_mut() = self.world.size.random_y(&mut self.rng);

        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        self.world.bullets.clear();
        self.world.enemies.clear();
    }

    /// Handles collisions between the player and the enemies
    fn handle_player_collisions(&mut self) {
        if self.world.enemies.iter().any(|enemy| self.world.player.collides_with(enemy)) {
            // Make an explosion where the player was
            let ppos = self.world.player.position();
            Game::make_explosion(&mut self.world.particles, ppos, 8);

            self.reset();
        }
    }

    // Generates a new explosion of the given intensity at the given position. This works best with values between 5 and 25
    fn make_explosion(particles: &mut Vec<Particle>, position: Point, intensity: u8) {
        for rotation in itertools::linspace(0.0, 2.0 * f64::consts::PI, 30) {
            for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
                particles.push(Particle::new(Pose::new(position.clone(), rotation), ttl));
            }
        }
    }

    #[allow(unused_variables)]
    pub fn spawn_circle_with_collision_colouring(&mut self, position: Point) {
        let mut entity = Entity::new();
        entity.velocity = Point::new(1., 1.);
        self.world.renderables.push(entity);
        // TODO
        //let player_position = self.world.player.position();
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
}
