use std::fmt;
use drawing::color;
use traits::Position;
use traits::Renderable;
use collision::Circle;
use collision::HitBoxes;

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;
use drawing::Point;

#[derive(Clone)]
pub struct Entity {
    position: Point,
    velocity: Point,
    hitbox: HitBoxes,
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Entity")
            .field("position", &self.position)
            .field("velocity", &self.velocity)
            .finish()
    }
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            position: Point::new(50., 50.),
            velocity: Point::new(50., 50.),
            hitbox: HitBoxes::Circle(Circle {
                radius: 15.,
                centre: Point::new(50., 50.),
            }),
        }
    }

    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        match self.hitbox {
            HitBoxes::Circle(circ) => {
                circ.draw(c, gl);
            },
            _ => {}
        }
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
