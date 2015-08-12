//! Collision detection tools

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

struct CircleHitbox {
    radius: f32,
    position: Point,
}

fn collision_between_circles(a: CircleHitbox, b:CircleHitbox) -> bool {
    false
}
