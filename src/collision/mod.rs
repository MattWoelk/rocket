//! Collision detection tools

#![allow(dead_code)]
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
    fn point_is_to_the_left(&self, p: Point) -> bool {
        let v1 = self.b - self.a;
        let v2 = p - self.a;
        v1.cross(v2) > 0.
            // TODO: verify if this is backward or not

        // TODO: Could this be done with dot product itself?
        //let line_to_point = LineSegment{a: self.a, b: p};
        //let angle = self.angle_to_line(line_to_point);
        //angle < TAU/2.
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

impl Collidable for Circle {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        self.centre.distance_to_point(circle.centre) < self.radius + circle.radius
    }

    fn collide_with_point(&self, point: Point) -> bool {
        return self.centre.distance_to_point(point) < self.radius
    }
}

impl Collidable for AABB {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        unimplemented!()
    }

    fn collide_with_point(&self, point: Point) -> bool {
        unimplemented!()
        // TODO: use the new point_is_to_the_left() one each line
        //       of the rectangle to see if the point lies within
        // TODO: !!!! May run into problems with equality,
        //       because if one is on the line, but the rest are in or out,
        //       then it might not register them as all being the same...

    }
}
