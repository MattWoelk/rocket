use drawing::color;
use super::Pose;
use traits::{Advance, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;
//use traits::Entity;
use maths::Point;

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
#[derive(Default, Clone)]
pub struct Particle {
    pub vector: Pose,
    pub ttl: f64
}

derive_position_direction!(Particle);

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Pose, ttl: f64) -> Particle {
        Particle { vector: vector, ttl: ttl }
    }

    /// Draw the particle
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        let radius = 5.0 * self.ttl;
        Ellipse::new(color::VIOLET).draw(
            [self.x() - radius, self.y() - radius, radius * 2.0, radius * 2.0],
            &c.draw_state, c.transform, gl);
    }

    /// Update the particle
    pub fn update(&mut self, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        let speed = 500.0 * self.ttl * self.ttl;
        self.advance(elapsed_time * speed);
    }
}

//impl Entity for Particle {
//    fn get_position(&mut self) -> Point {
//        Point::new(1., 1.)
//    }
//}
