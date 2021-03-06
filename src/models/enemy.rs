use drawing::Point;
use super::Vector;
use traits::{Advance, Collide, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

/// Enemies follow the player in order to cause a collision and let him explode 
pub struct Enemy {
    vector: Vector
}

derive_position_direction!(Enemy);

impl Enemy {
    /// Create a enemy with the given vector
    pub fn new(vector: Vector) -> Enemy {
        Enemy { vector: vector }
    }

    /// Draw the enemy
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        Ellipse::new([1.0, 1.0, 0.0, 1.0]).draw(
            [self.x() - 10.0, self.y() - 10.0, 20.0, 20.0],
            &c.draw_state, c.transform, gl);
    }

    /// Update the enemy
    pub fn update(&mut self, speed: f64, player_position: Point) {
        // Point to the player
        self.point_to(player_position);
        self.advance(speed);
    }
}

impl Collide for Enemy {
    fn radius(&self) -> f64 { 10.0 }
}
