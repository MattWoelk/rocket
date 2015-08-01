use std::f64;

use rand::Rng;

use drawing::{Point, Size};

/// A `Pose`
#[derive(Clone, Default)]
pub struct Pose {  // TODO: Why isn't this magnitude and direction?
                     // why position and direction?
                     // it should just be position on its own!!!!!
                     // This makes no sense!!!
                     // Is the position where it is, and the direction where it's going?
                     // If so, this should really be two vectors, as the speed should be stored
                     // with the velocity
    /// The position of the vector
    pub position: Point,
    /// The direction angle, in radians
    pub angle_radians: f64
}

impl Pose {
    /// Returns a new `Pose`
    pub fn new(position: Point, angle_radians: f64) -> Pose {
        Pose { position: position, angle_radians: angle_radians }
    }

    /// Returns a random `Pose` within the given bounds
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Pose {
        Pose::new(Point::random(rng, bounds), rng.gen())
    }

    /// Consumes the vector and returns a new one with inverted direction
    pub fn invert(mut self) -> Pose {
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
