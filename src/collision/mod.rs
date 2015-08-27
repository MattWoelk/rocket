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



pub trait Collidable {
    fn collide_with_circle(&self, &Circle) -> bool;
    fn collide_with_point<A>(&self, point: A) -> bool where A: Into<Point>;
    // TODO: add each new type here, and then all will be enforced.
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Circle {
    pub radius: f64,
    pub centre: Point,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    pub fn new<A>(p1: A, p2: A) -> LineSegment
        where A: Into<Point>
    {
        LineSegment {
            a: p1.into(),
            b: p2.into(),
        }
    }

    /// If left of line: > 0, if right of line: < 0, if on line: == 0.
    // TODO: make this return an enum, rather than cryptic numbers.
    // Can I get fancy with generics so I can say == left_or_within ?
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

pub struct ConvexPolygon {
    points: Box<Vec<Point>>,
}

impl ConvexPolygon {
    pub fn new<A>(points: Vec<A>) -> Self
        where A: Into<Point>
    {
        ConvexPolygon {
            points: Box::new(points.into_iter().map(|x| {
                let b: Point = x.into();
                b
            }).collect()),
        }
    }

    //fn lines(&self) -> Vec<&[&Point]> {
    //    let len = self.points.len();
    //    let points_with_extra: Vec<&Point> = self.points.iter().cycle().take(len + 1).collect();
    //    let lines = points_with_extra.clone().windows(2).into_iter();
    //    lines.collect()
    //}
}

pub struct Arc {
    angle_start: f64,
    angle_end: f64,
    thickness: f64,
    inner_circle: Circle,
}

impl Arc {
    pub fn new<A>(angle_start: f64, angle_end: f64, thickness: f64, circle: A) -> Arc
        where A: Into<Circle>
    {
        Arc {
            angle_start: angle_start,
            angle_end: angle_end,
            thickness: thickness,
            inner_circle: circle.into(),
        }
    }
}

impl Collidable for Circle {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        self.centre.distance_to_point(circle.centre) < self.radius + circle.radius
    }

    fn collide_with_point<A>(&self, point: A) -> bool
        where A: Into<Point>
    {
        self.centre.distance_to_point(point.into()) < self.radius
    }
}

impl Collidable for ConvexPolygon {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        unimplemented!()
    }

    fn collide_with_point<A>(&self, point: A) -> bool
        where A: Into<Point>
    {
        let len = self.points.len();
        let pt = point.into();

        // TODO: make a ConvexPolygon method that gives lines using this method.
        let points_with_extra: Vec<&Point> = self.points.iter().cycle().take(len + 1).collect();
        let lines: Vec<LineSegment> = points_with_extra.windows(2).map(|x| LineSegment::new(x[0].clone(), x[1].clone())).collect();

        let sides: Vec<f64> = lines.iter().map(|x| x.point_is_on_side(pt)).collect();

        // TODO: optimize this to make a decision as we go using .fold() or similar
        // though this may be pretty quick already....
        sides.iter().all(|&x| x <= 0.) || sides.iter().all(|&x| x >= 0.)
    }
}

impl Collidable for Arc {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        unimplemented!()
    }

    fn collide_with_point<A>(&self, point: A) -> bool
        where A: Into<Point>
    {
        let pt = point.into();
        let outer_circle = Circle {
            radius: self.inner_circle.radius + self.thickness,
            centre: self.inner_circle.centre
        };

        let angle_difference = (self.angle_end + TAU) - (self.angle_end + TAU);
        let line_start = LineSegment::new(
            Point::new(self.angle_start.cos() + self.inner_circle.centre.x, self.angle_start.sin() + self.inner_circle.centre.y),
            self.inner_circle.centre
        );
        let line_end = LineSegment::new(
            Point::new(self.angle_end.cos(), self.angle_end.sin()),
            self.inner_circle.centre
        );

        let within_angles = if angle_difference > TAU/2. {
            line_start.point_is_on_side(pt) <= 0. ||
                line_end.point_is_on_side(pt) >= 0.
        } else {
            line_start.point_is_on_side(pt) <= 0. &&
                line_end.point_is_on_side(pt) >= 0.
        };

        !self.inner_circle.collide_with_point(pt) &&
            outer_circle.collide_with_point(pt) &&
            within_angles
    }
}

#[cfg(test)]
mod test {
    use super::{LineSegment, ConvexPolygon, Arc, Circle, TAU, Collidable};
    use drawing::Point;

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

    #[test]
    fn test_contex_polygon() {
        let contex_polygon = ConvexPolygon::new(vec![(0., 0.), (4., 0.), (4., 4.)]);

        assert_eq!(contex_polygon.collide_with_point((2., 1.)), true);
        assert_eq!(contex_polygon.collide_with_point((1., 1.)), true);
        assert_eq!(contex_polygon.collide_with_point((-1., -1.)), false);
        assert_eq!(contex_polygon.collide_with_point((0.5, 0.)), true);
    }

    #[test]
    fn test_point_in_arc() {
        let arc = Arc::new(0., TAU/4., 5., Circle {
            radius: 5.,
            centre: Point::new(0., 0.),
        });

        assert_eq!(arc.collide_with_point((5.1, 0.)), true);
        assert_eq!(arc.collide_with_point((10.1, 0.)), false);
    }
}
