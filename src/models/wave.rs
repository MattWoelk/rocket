use maths::{TAU, Point};
use super::Pose;
use traits::{Advance, Collide, Position};
//use traits::Entity;

use graphics::{Context, Line};
use graphics::math::Vec2d;
use opengl_graphics::GlGraphics;

const WAVE_SPEED_PER_SECOND: f64 = 500.;

/// Enemies follow the player in order to cause a collision and let him explode
#[derive(Clone)]
pub struct Wave {
    position: Point,
    vector: Pose,
    pub radius: f64,
    angle_segments: Vec<[f64; 2]>,
    colour: [f32; 4],
    line_width: f64,
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
            angle_segments: vec![
                [TAU * 1./8., TAU * 2./8.],
                [TAU * 3./8., TAU * 4./8.],
                [TAU * 5./8., TAU * 6./8.],
                [TAU * 7./8., TAU * 8./8.]
            ],
            colour: [0.5, 1.0, 0.0, 1.0],
            line_width: 8.0,
        }
    }

    pub fn new_grass(position: Point) -> Wave {
        let mut wave = Wave::new(position);
        wave.colour = [0.5, 1.0, 0.0, 1.0];
        wave
    }

    pub fn new_fire(position: Point) -> Wave {
        let mut wave = Wave::new(position);
        wave.colour = [1.0, 0.0, 0.5, 1.0];
        wave
    }

    pub fn new_water(position: Point) -> Wave {
        let mut wave = Wave::new(position);
        wave.colour = [0.0, 0.5, 1.0, 1.0];
        wave
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
                    Point::new_by_radius_angle(self.radius, angle).translate(self.position)
                });

            let vertices = points
                .map(|p| Vec2d::from(p))
                .collect::<Vec<Vec2d>>();

            for pair in vertices.windows(2) {
                let line = [pair[0][0], pair[0][1], pair[1][0], pair[1][1]];
                Line::new(self.colour, self.line_width)
                    .draw(line,
                          &c.draw_state,
                          c.transform,
                          gl);
            }
        }
    }

    /// Update the wave
    pub fn update(&mut self, dt: f64) {
        self.radius += WAVE_SPEED_PER_SECOND * dt;
    }
}

impl Collide for Wave {
    fn radius(&self) -> f64 { self.radius }
}

//impl Entity for Wave {
//    fn get_position(&mut self) -> Point {
//        Point::new(1., 1.)
//    }
//}
