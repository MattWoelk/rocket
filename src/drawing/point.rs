#![allow(dead_code)]

use rand::Rng;
use std::ops::{Add, Sub};

use super::Size;
use graphics::math::Vec2d;

/// A `Point` represents a position in space
#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    /// Returns a new `Point` with the given coordinates
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }

    pub fn new_by_radius_angle(radius: f64, angle:f64) -> Point {
        Point {
            x: radius * angle.sin(),
            y: radius * angle.cos(),
        }
    }

    /// Returns a random `Point` within the given bounds (exclusive)
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Point {
        Point {
            x: rng.gen_range(0.0, bounds.width),
            y: rng.gen_range(0.0, bounds.height)
        }
    }

    /// Returns the squared distance from this point to the given one
    pub fn squared_distance_to(&self, target: Point) -> f64 {
        (self.x - target.x) * (self.x - target.x)
        + (self.y - target.y) * (self.y - target.y)
    }

    pub fn distance_to_point(&self, point: Point) -> f64 {
        point.squared_distance_to(point).sqrt()
    }

    /// Rotates the point through the origin in the given angle (radians)
    pub fn rotate(mut self, radians: f64) -> Point {
        let radius = (self.x * self.x + self.y * self.y).sqrt();
        let point_angle = (self.y / self.x).atan();
        let final_angle = point_angle + radians;
        self.x = final_angle.cos() * radius;
        self.y = final_angle.sin() * radius;
        self
    }

    /// Translates the point by another point
    pub fn translate(mut self, other: Point) -> Point {
        self.x += other.x;
        self.y += other.y;
        self
    }

    pub fn translated(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn normal(&self) -> Point {
        Point {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn of_length(&self, length: f64) -> Point {
        self.unit_vector().multiply_by_scalar(length)
    }

    pub fn unit_vector(&self) -> Point {
        self.divide_by_scalar(self.abs())
    }

    pub fn radians(self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn dot(&self, p: Point) -> f64 {
        (self.x * p.x) + (self.y * p.y)
    }

    pub fn cross(&self, p: Point) -> f64 {
        (self.x * p.y) - (self.y * p.x)
    }

    pub fn abs(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn multiply_by_scalar(&self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn divide_by_scalar(&self, scalar: f64) -> Point {
        Point {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    pub fn invert(&self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<Point> for Vec2d {
    fn from(p: Point) -> Self {
        [p.x, p.y]
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
