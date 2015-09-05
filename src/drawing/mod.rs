//! Helper objects and constants

extern crate graphics;

mod point;
mod size;

use traits::Renderable;
use collision::Circle;
use opengl_graphics::GlGraphics;
use graphics::{Context, Ellipse};
use models::CollisionTestBall;

pub type Color = [f32; 4];

pub mod color {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
}

pub use self::point::Point;
pub use self::size::Size;


impl Renderable for Circle {
    fn draw(&self, c: &graphics::context::Context, gl: &mut GlGraphics) {
        Ellipse::new(color::BLUE).draw(
            [self.centre.x, self.centre.y, self.radius, self.radius],
            &c.draw_state, c.transform, gl);
    }

    //TODO: remove this allow
    #[allow(unused_variables)]
    fn update_2(&mut self, units: f64, entities: &Vec<CollisionTestBall>, my_entity_index: i64, player_pos: Point) {

    }
}
