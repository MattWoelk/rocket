//! Collision detection tools

#![allow(dead_code)]
// TODO: ^ get rid of this

use graphics::math::Vec2d;
use drawing::Point;

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
    // TODO: add each new type here, and then all will be enforced.
}

struct Circle {
    radius: f64,
    centre: Point,
}

impl Collidable for Circle {
    fn collide_with_circle(&self, circle: &Circle) -> bool {
        self.centre.distance_to_point(&circle.centre) < self.radius + circle.radius
    }
}
