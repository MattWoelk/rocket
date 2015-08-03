use std::f64;

use drawing::Point;
use super::Pose;
use traits::{Advance, Collide, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

const TAU: f64 = f64::consts::PI * 2.;

/// Enemies follow the player in order to cause a collision and let him explode
pub struct Wave {
    position: Point,
    vector: Pose,
    radius: f64,
    angle_segments: Vec<[f64; 2]>,
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
            angle_segments: vec![[TAU * 1./6., TAU * 1./3.]],
        }
    }

    /// Draw the wave
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        let sides_per_one_radian = 128. / TAU;

        for segment in &self.angle_segments {
            let angle_indices_in_range = ((segment[1] - segment[0]) * sides_per_one_radian) as i64;
            let range_of_angle_indices = 0..(angle_indices_in_range + 1);
            let angles = range_of_angle_indices.map(|x| x as f64 * (segment[1] - segment[0]) / angle_indices_in_range as f64 + segment[0]);

            let points = angles.map(
                |angle| {
                    Point::new_by_radius_angle(self.radius, angle).translate(&self.position)
                });

            for point in points {
                let y = point.y;
                let x = point.x;
                let r = 1.;

                Ellipse::new([1.0, 0.0, 0.0, 1.0]).draw(
                    [
                        x - r + 10.0,
                        y - r + 10.0,
                        2.0 * r - 20.0,
                        2.0 * r - 20.0
                    ],
                    &c.draw_state, c.transform, gl);
            }
        }
    }

    /// Update the wave
    pub fn update(&mut self, dt: f64) {
        // Point to the player
        self.radius += 100.0 * dt;
    }
}

impl Collide for Wave {
    fn radius(&self) -> f64 { self.radius }
}
