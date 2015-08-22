//! Collision detection tools

#![allow(dead_code, unused_variables)]
// TODO: ^ get rid of this

use std::f64;
use graphics::math::Vec2d;
use drawing::Point;

const TAU: f64 = f64::consts::PI * 2.;


struct ContactPoint {
    position: Vec2d,
    penetration: f32,
}

struct ContactManifold {
    point_count: i32,
    points: [ContactPoint; 2],
    normal: Vec2d,
}



trait Collidable {
    fn collide_with_circle(&self, &Circle) -> bool;
    fn collide_with_point(&self, Point) -> bool;
    // TODO: add each new type here, and then all will be enforced.
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Circle {
    radius: f64,
    centre: Point,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    fn new(p1: Point, p2: Point) -> LineSegment {
        LineSegment {
            a: p1,
            b: p2,
        }
    }

    /// If left of line: > 0, if right of line: < 0, if on line: == 0.
    fn point_is_on_side(&self, p: Point) -> f64 {
        let v1 = self.b - self.a;
        let v2 = p - self.a;

        v1.cross(v2)
    }

    fn angle_to_line(&self, line: LineSegment) -> f64 {
        let self_vector = self.b - self.a;
        let line_vector = line.b - line.a;

        let numerator = self_vector.dot(line_vector);
        let denominator = self_vector.abs() * line_vector.abs();
        (numerator / denominator).acos()
    }
}

impl Into<LineSegment> for (f64, f64, f64, f64) {
    fn into(self) -> LineSegment {
        LineSegment{
            a: Point { x: self.0, y: self.1 },
            b: Point { x: self.2, y: self.3 },
        }
    }
}

impl Collidable for Circle {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        self.centre.distance_to_point(circle.centre) < self.radius + circle.radius
    }

    fn collide_with_point(&self, point: Point) -> bool {
        return self.centre.distance_to_point(point) < self.radius
    }
}

#[test]
fn test_sides() {
    let line: LineSegment = (0., 0., 5., 5.).into();
    let point_left_of_line: Point = (1., 9.).into();
    assert_eq!(line.point_is_on_side(point_left_of_line), 40.);

    let point_right_of_line: Point = (9., 1.).into();
    assert_eq!(line.point_is_on_side(point_right_of_line), -40.);

    let point_on_line: Point = (5., 5.).into();
    assert_eq!(line.point_is_on_side(point_on_line), 0.);
}

#[test]
fn test_point_in_circle() {
    let circle = Circle { radius: 5., centre: Point::new(0., 0.) };

    assert_eq!(circle.collide_with_point(Point::new(1., 1.)), true);
    assert_eq!(circle.collide_with_point(Point::new(5., -5.)), false);
}
