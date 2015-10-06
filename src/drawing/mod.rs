//! Helper objects and constants

extern crate graphics;

mod size;

use rand::Rng;
use traits::Renderable;
use maths::{Circle, Point};
use opengl_graphics::GlGraphics;
use graphics::{Context, Ellipse};
use models::Entity;

pub type Color = [f32; 4];

pub mod color {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
}

pub use self::size::Size;


impl Renderable for Circle {
    fn draw(&self, c: &graphics::context::Context, gl: &mut GlGraphics) {
        Ellipse::new(color::BLUE).draw(
            [self.centre.x, self.centre.y, self.radius, self.radius],
            &c.draw_state, c.transform, gl);
    }

    //TODO: remove this allow
    #[allow(unused_variables)]
    fn update_2(&mut self, units: f64, entities: &Vec<Entity>, my_entity_index: i64, player_pos: Point) {

    }
}

impl Point {
    /// Returns a random `Point` within the given bounds (exclusive)
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Point {
        Point {
            x: rng.gen_range(0.0, bounds.width),
            y: rng.gen_range(0.0, bounds.height)
        }
    }
}
