use drawing::Point;
use super::Pose;
use traits::{Advance, Collide, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

/// Enemies follow the player in order to cause a collision and let him explode
pub struct Wave {
    position: Point,
    vector: Pose,
    radius: f64,
}

derive_position_direction!(Wave);

impl Wave {
    /// Create a wave with the given vector
    pub fn new(position: Point) -> Wave {
        let vector = Pose::new(position.clone(), 0.0);
        Wave {
            position: position,
            vector: vector,
            radius: 30.0,
        }
    }

    /// Draw the wave
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        Ellipse::new([0.0, 1.0, 0.0, 1.0]).draw(
            [self.x() - self.radius, self.y() - self.radius, 2.0 * self.radius, 2.0 * self.radius],
            &c.draw_state, c.transform, gl);
    }

    /// Update the wave
    pub fn update(&mut self, speed: f64, player_position: Point) {
        // Point to the player
    }
}

impl Collide for Wave {
    fn radius(&self) -> f64 { self.radius }
}
