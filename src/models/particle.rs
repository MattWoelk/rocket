use drawing::color;
use super::PositionAndDirection;
use traits::{Advance, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub position_and_direction: PositionAndDirection,
    pub ttl: f64
}

derive_position_direction!(Particle);

impl Particle {
    /// Create a particle with the given position_and_direction and time to live in seconds
    pub fn new(position_and_direction: PositionAndDirection, ttl: f64) -> Particle {
        Particle { position_and_direction: position_and_direction, ttl: ttl }
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
