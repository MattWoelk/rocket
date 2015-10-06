use models::Entity;
use maths::Point;
use maths::HitBoxes;
use maths::Circle;
use drawing::{color, Color};

//static orb_prototype: Entity;
pub static OrbPrototype: Entity = Entity {
    position: Point { x: 50., y: 50. },
    velocity: Point { x: 50., y: 50. },
    hitbox: HitBoxes::Circle(Circle {
        radius: 15.,
        centre: Point { x: 50., y: 50. },
    }),
    color: color::BLUE,
};

fn do_thing() {
    // more like do NOT, hahaaa.
}




// attempt in rust playgrount:
// fn main() {
//     println!("Hello, world!");
//
//     let h = H { f: do_thing };
//
//     println!("{:#?}", h);
// }
//
// fn do_thing(a:H, b:H) -> String {
//     "STR".to_string()
// }
//
//
// struct H {
//     f: (Fn(H, H) -> String),
// }
