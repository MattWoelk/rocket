use std::f64;

use rand::Rng;

use drawing::{Point, Size};

/// A `PositionAndDirection`
#[derive(Clone, Default)]
pub struct PositionAndDirection {
    /// The position of the position_and_direction
    pub position: Point,
    /// The direction angle, in radians
    pub direction: f64
}

impl PositionAndDirection {
    /// Returns a new `PositionAndDirection`
    pub fn new(position: Point, direction: f64) -> PositionAndDirection {
        PositionAndDirection { position: position, direction: direction }
    }

    /// Returns a random `PositionAndDirection` within the given bounds
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> PositionAndDirection {
        PositionAndDirection::new(Point::random(rng, bounds), rng.gen())
    }

    /// Consumes the PositionAndDirection and returns a new one with inverted direction
    pub fn invert(mut self) -> PositionAndDirection {
        self.direction -= f64::consts::PI;
        self
    }
}

/// A macro to implement `Position` and `Direction` for any type that has a field named
/// `position_and_direction`
#[macro_export]
macro_rules! derive_position_direction {
    ($t:ty) => {
        impl ::traits::Position for $t {
            fn x(&self) -> f64 { self.position_and_direction.position.x }
            fn x_mut(&mut self) -> &mut f64 { &mut self.position_and_direction.position.x }
            fn y(&self) -> f64 { self.position_and_direction.position.y }
            fn y_mut(&mut self) -> &mut f64 { &mut self.position_and_direction.position.y }
        }

        impl ::traits::Advance for $t {
            fn direction(&self) -> f64 {
                self.position_and_direction.direction
            }

            fn direction_mut(&mut self) -> &mut f64 {
                &mut self.position_and_direction.direction
            }

        }

        impl $t {
            fn add_direction(&mut self, radians: f64) {
                self.position_and_direction.direction += radians;
            }
        }
    }
}
