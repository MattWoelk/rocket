use std::f64;

use rand::Rng;

use drawing::{Point, Size};

/// A `Vector`
#[derive(Clone, Default)]
pub struct Vector {
    /// The position of the vector
    pub position: Point,
    /// The direction angle, in radians
    pub angle_radians: f64
}

impl Vector {
    /// Returns a new `Vector`
    pub fn new(position: Point, angle_radians: f64) -> Vector {
        Vector { position: position, angle_radians: angle_radians }
    }

    /// Returns a random `Vector` within the given bounds
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Vector {
        Vector::new(Point::random(rng, bounds), rng.gen())
    }

    /// Consumes the vector and returns a new one with inverted direction
    pub fn invert(mut self) -> Vector {
        self.angle_radians -= f64::consts::PI;
        self
    }
}

/// A macro to implement `Position` and `Direction` for any type that has a field named `vector`
#[macro_export]
macro_rules! derive_position_direction {
    ($t:ty) => {
        impl ::traits::Position for $t {
            fn x(&self) -> f64 { self.vector.position.x }
            fn x_mut(&mut self) -> &mut f64 { &mut self.vector.position.x }
            fn y(&self) -> f64 { self.vector.position.y }
            fn y_mut(&mut self) -> &mut f64 { &mut self.vector.position.y }
        }

        impl ::traits::Advance for $t {
            fn angle_radians(&self) -> f64 {
                self.vector.angle_radians
            }

            fn direction_mut(&mut self) -> &mut f64 {
                &mut self.vector.angle_radians
            }
        }
    }
}
