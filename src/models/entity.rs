use drawing::color;
use traits::Position;

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;
use drawing::Point;

#[derive(Clone, Debug)]
pub struct Entity {
    position: Point,
    velocity: Point,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            position: Point::new(0., 0.),
            velocity: Point::new(0., 0.),
        }
    }

    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        Ellipse::new(color::BLUE).draw(
            [self.position.x, self.position.y, 5., 5.],
            &c.draw_state, c.transform, gl);
    }

    // TODO: remove this allow
    #[allow(unused_variables)]
    pub fn update_2(&mut self, units: f64, entities: &Vec<Entity>) {
        //self.advance(units);
        for entity in entities {
            // TODO: if any are touching me, change my colour
            //println!("Entity: {:#?}", entity);
        }
    }
}
