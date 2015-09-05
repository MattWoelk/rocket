use graphics::{Context, Polygon, Transformed};
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::{color, Point, Size};
use super::Pose;
use traits::{Advance, Collide, Position};

pub const PLAYER_DEFAULT_SPEED: f64 = 200.;

/// The `Player` is the rocket controlled by the user
#[derive(Default, Clone)]
pub struct Player {
    // TODO: make this "position" and "velocity" instead.
    pub vector: Pose,
    pub speed: f64,
}

derive_position_direction!(Player);

/// The player is drawn as the triangle below
const POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Player {
        Player {
            vector: Pose::random(rng, bounds),
            speed: PLAYER_DEFAULT_SPEED,
        }
    }

    /// Draw the player
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        // Set the center of the player as the origin and rotate it
        let transform = c.transform.trans(self.x(), self.y())
                                   .rot_rad(self.angle_radians());

        // Draw a rectangle on the position of the player
        Polygon::new(color::RED).draw(POLYGON, &c.draw_state, transform, gl);
    }

    /// Returns the nose of the rocket
    pub fn nose(&self) -> Point {
        Point::new(POLYGON[1][0], POLYGON[1][1])
            .rotate(self.angle_radians())
            .translate(self.position())
    }
}

impl Collide for Player {
    fn radius(&self) -> f64 { 6.0 }
}
