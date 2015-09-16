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
//use models::level_0::{Timers, Actions};
use models::Controls;
use traits::{Advance, Collide, Position, Entity, Level};
use models::CollisionTestBall;

use sdl2::controller::{Axis, Button};

pub const BULLET_RATE: f64 = 0.3;

/// The data structure that drives the game
pub struct Game {
    /// The level contains everything that needs to be drawn
    pub level: Box<Level + 'static>,
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
    pub fn new<L:Level + 'static>(size: Size, level: L) -> Game {
        let mut rng = rand::thread_rng();
        Game {
            level: Box::new(level),
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
        let control: Controls = match key {
            Key::Left => {
                if pressed {
                    Controls::X1(-32768)
                } else {
                    Controls::X1(0)
                }
            },
            Key::Right => {
                if pressed {
                    Controls::X1(32768)
                } else {
                    Controls::X1(0)
                }
            },
            Key::Up => {
                if pressed {
                    Controls::Y1(-32768)
                } else {
                    Controls::Y1(0)
                }
            },
            Key::Down => {
                if pressed {
                    Controls::Y1(32768)
                } else {
                    Controls::Y1(0)
                }
            },
            Key::Space => Controls::X(pressed),
            _ => Controls::None
        };
        self.level.handle_control(control);
    }

    pub fn button_press(&mut self, button: Button) {
        self.handle_button(button, true);
    }

    pub fn button_release(&mut self, button: Button) {
        self.handle_button(button, false);
    }

    fn handle_button(&mut self, button: Button, pressed: bool) {
        let control = match button {
            Button::A => Controls::A(pressed),
            Button::B => Controls::B(pressed),
            Button::X => Controls::X(pressed),
            _ => Controls::None
        };
        self.level.handle_control(control);
    }

    pub fn handle_axis(&mut self, axis: Axis, value: i32) {
        // TODO: set the dead zone based on the magnitude instead of the single axis value
        // will require handling both axes at once.
        let dead_zoned_value = if value.abs() < 5000 {0} else {value - (5000 * value.signum())};

        let control = match axis {
            Axis::LeftX => Controls::X1(dead_zoned_value as i64),
            Axis::LeftY => Controls::Y1(dead_zoned_value as i64),
            _ => Controls::None
        };
        self.level.handle_control(control);
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update(&mut self, dt: f64) {
        self.level.update(
            &mut self.particles,
            &mut self.player,
            &mut self.waves,
            &mut self.enemies,
            &self.size,
            dt);
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
        //self.level.score += 10 * killed_enemies;
    }

    // Generates a new explosion of the given intensity at the given position. This works best with values between 5 and 25
    pub fn make_explosion(particles: &mut Vec<Particle>, position: Point, intensity: u8) {
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
