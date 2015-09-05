use drawing::color;
use super::Pose;
use traits::{Advance, Collide, Position};
use traits::Entity;
use drawing::Point;

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
#[derive(Clone)]
pub struct Bullet {
    vector: Pose
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Pose) -> Bullet {
        Bullet { vector: vector }
    }

    /// Draw the bullet
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        Ellipse::new(color::BLUE).draw(
            [self.x() - self.radius(), self.y() - self.radius(), self.diameter(), self.diameter()],
            &c.draw_state, c.transform, gl);
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f64) {
        self.advance(units);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f64 { 3.0 }
}

impl Entity for Bullet {
    fn get_position(&mut self) -> Point {
        Point::new(1., 1.)
    }
}
