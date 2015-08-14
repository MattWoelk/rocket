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

// TODO: figure out if all the combinations of collisions
//       should be in an enum, or trait, or something different?
//       - an enum with structs in it!
enum Collidable {
    Circle { radius: f64, centre: Point },
//    Capsule2D,
//    Hull2D,
//    Mesh2D,
}

//struct CircleHitbox {
//    radius: f64,
//    centre: Point,
//}

fn collides(a: Collidable, b: Collidable) -> bool {
    match a {
        Collidable::Circle {radius:r, centre:c} => {
            match b {
                Collidable::Circle {radius:r2, centre:c2} => {
                    c.distance_to_point(&c2) < r + r2
                }
            }
        }
    }
}
