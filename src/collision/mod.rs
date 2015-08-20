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
struct Circle {
    radius: f64,
    centre: Point,
}

#[derive(Clone, Copy, Default, Debug)]
struct LineSegment {
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

    fn new_from_xy_xy(x1: f64, y1: f64, x2: f64, y2: f64) -> LineSegment {
        LineSegment {
            a: Point{x:x1, y:y1},
            b: Point{x:x2, y:y2},
        }
    }

    /// If left of line: > 0, if right of line: < 0, if on line: == 0.
    fn point_is_on_side(&self, p: Point) -> f64 {
        let v1 = self.b - self.a;
        let v2 = p - self.a;

        let p1 = self.a;
        let p2 = self.b;
        let p3 = p;

        v1.cross(v2)

        // TODO: New algorithm: dot product of the NORMAL line to the line,
        // and a line from any point on the line to the p Point.
        // This will return an f64 whose sign is useful to determine the side.
        // This should help resolve the problem where this is used below.
    }

    fn angle_to_line(&self, line: LineSegment) -> f64 {
        // TODO: Instead, maybe we should be using vec2d instead of points,
        // and then subtract vectors more abstractly.
        let self_vector = Point {
            x: self.b.x - self.a.x,
            y: self.b.y - self.a.y,
        };

        let line_vector = Point {
            x: line.b.x - line.a.x,
            y: line.b.y - line.a.y,
        };

        let numerator = self_vector.dot(line_vector);
        let denominator = self_vector.abs() * line_vector.abs();
        (numerator / denominator).acos()
    }
}

/// TODO: This may not be required, as the general equation for a polygon
/// should do the job just fine.
#[derive(Clone, Copy, Default, Debug)]
struct AABB {
    centre: Point,
    half_width: f64,
    half_height: f64,
}

impl AABB {
    fn top_left(&self) -> Point {
        Point{
            x:(self.centre.x - self.half_width),
            y:(self.centre.y + self.half_height),
        }
    }

    fn top_right(&self) -> Point {
        Point{
            x:(self.centre.x + self.half_width),
            y:(self.centre.y + self.half_height),
        }
    }

    fn bottom_right(&self) -> Point {
        Point{
            x:(self.centre.x + self.half_width),
            y:(self.centre.y - self.half_height),
        }
    }

    fn bottom_left(&self) -> Point {
        Point{
            x:(self.centre.x - self.half_width),
            y:(self.centre.y - self.half_height),
        }
    }
}

/// TODO: This is no longer required, as I've found a better way
/// to check for a point in an arc
struct IsoscelesTriangle {
    peak: Point,
    middle_of_base: Point,
    width: f64,
}

impl IsoscelesTriangle {
    fn three_points(&self) -> (Point, Point, Point) {
        let centre_vector = self.middle_of_base - self.peak;
        let perpendicular_line = centre_vector.normal();
        let p1 = self.middle_of_base.translated(perpendicular_line.unit_vector().multiply_by_scalar(self.width / 2.));
        let p2 = self.middle_of_base.translated(perpendicular_line.unit_vector().multiply_by_scalar(self.width / -2.));

        (
            self.peak,
            p1,
            p2,
        )
    }

    fn new_from_angle_peak_and_height(angle: f64, peak: Point, height: f64) -> IsoscelesTriangle {
        unimplemented!()
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

/// TODO: This may not be required, as the general equation for a polygon
/// should do the job just fine.
impl Collidable for AABB {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        unimplemented!()
    }

    fn collide_with_point(&self, point: Point) -> bool {
        unimplemented!()
        // TODO: use the new point_is_on_side() one each line
        //       of the rectangle to see if the point lies within
        // TODO: !!!! May run into problems with equality,
        //       because if one is on the line, but the rest are in or out,
        //       then it might not register them as all being the same...

    }
}

/// TODO: This is no longer required, as I've found a better way
/// to check for a point in an arc
impl Collidable for IsoscelesTriangle {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        unimplemented!()
    }

    fn collide_with_point(&self, point: Point) -> bool {
        // TODO: This should be rewritten generically
        //       so it can be reused for any polygon.
        let (p1, p2, p3) = self.three_points();
        let winding_1 = LineSegment::new(p1, p2).point_is_on_side(point);
        let winding_2 = LineSegment::new(p2, p3).point_is_on_side(point);
        let winding_3 = LineSegment::new(p3, p1).point_is_on_side(point);

        (winding_1 >= 0. && winding_2 >= 0. && winding_3 >= 0.) ||
        (winding_1 <= 0. && winding_2 <= 0. && winding_3 <= 0.)
    }
}

#[test]
fn test_sides() {
    let line = LineSegment::new_from_xy_xy(0., 0., 5., 5.);
    let point_left_of_line = Point::new(1., 9.);
    assert_eq!(line.point_is_on_side(point_left_of_line), 40.);

    let point_right_of_line = Point::new(9., 1.);
    assert_eq!(line.point_is_on_side(point_right_of_line), -40.);

    let point_on_line = Point::new(5., 5.);
    assert_eq!(line.point_is_on_side(point_on_line), 0.);
}
