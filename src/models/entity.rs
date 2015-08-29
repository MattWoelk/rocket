use std::fmt;
use drawing::{color, Color};
use traits::Position;
use traits::Renderable;
use collision::Circle;
use collision::HitBoxes;
use collision::Collidable;

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;
use drawing::Point;

#[derive(Clone)]
pub struct Entity {
    pub position: Point,
    pub velocity: Point,
    pub hitbox: HitBoxes,
    pub color: Color,
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
            color: color::BLUE,
        }
    }

    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        match self.hitbox {
            HitBoxes::Circle(circ) => {
                circ.draw(c, gl);
            },
            _ => {}
        }
        Ellipse::new(self.color).draw(
            [self.position.x, self.position.y, 35., 35.],
            &c.draw_state, c.transform, gl);
    }

    // TODO: remove this allow
    #[allow(unused_variables)]
    pub fn update_2(&mut self, units: f64, entities: &Vec<Entity>, my_entity_index: i64, player_pos: Point) {
        //self.advance(units);
        self.position = self.position + self.velocity;
        match &mut self.hitbox {
            &mut HitBoxes::Circle(ref mut circ) => {
                circ.centre = self.position;
                self.color = color::BLUE;

                if circ.collide_with_point(player_pos) {
                    self.color = color::RED;
                }

                for (i, entity) in entities.iter().enumerate() {
                    if i as i64 == my_entity_index {
                        continue;
                    }

                    match entity.hitbox {
                        HitBoxes::Circle(circ) => {
                            if circ.collide_with_circle(&circ) {
                                self.color = color::RED;
                                break;
                            }
                        },
                        _ => {}
                    }
                    // TODO: if any are touching me, change my colour
                    //println!("Entity: {:#?}", entity);
                }
            },
            _ => {}
        }
        for entity in entities {
            // TODO: if any are touching me, change my colour
            //println!("Entity: {:#?}", entity);
        }
    }
}
