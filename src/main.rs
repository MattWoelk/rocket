extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event::{Event, Events, EventLoop};
use piston::window::WindowSettings;

use std::path::Path;
use graphics::{Context, Polygon};
use graphics::math::Vec2d;
use graphics::{Transformed};
use opengl_graphics::glyph_cache::GlyphCache;

use std::f64;
const TAU: f64 = f64::consts::PI * 2.;

fn main() {
    // Initialization stuff
    let opengl = OpenGL::_3_2;

    let window: GlutinWindow =
        WindowSettings::new("Rocket", [1024, 600])
            .exit_on_esc(true)
            .opengl(opengl)
            .samples(8)
            .into();

    let mut gl = GlGraphics::new(opengl);

    // Event handling
    for e in window.events().ups(60).max_fps(60) {
        match e {
            Event::Render(args) => {
                gl.draw(args.viewport(), |c, g| draw_polygon(&c, g));
            }

            _ => {}
        }
    }
}


fn draw_polygon(c: &Context, gl: &mut GlGraphics) {
    let angle_segments = vec![[TAU * 1./6., TAU * 1./3.]];
    let sides_per_one_radian = 32. / TAU;
    let radius = 500.0;
    let position = Point::new(00.0, 300.0);

    for segment in &angle_segments {
        let angle_indices_in_range = ((segment[1] - segment[0]) * sides_per_one_radian) as i64;
        let range_of_angle_indices = 0..(angle_indices_in_range + 1);
        let angles = range_of_angle_indices.map(|x| x as f64 * (segment[1] - segment[0]) / angle_indices_in_range as f64 + segment[0]);

        let angles_vec = angles.collect::<Vec<_>>();

        let outer_points = angles_vec.iter().map(
            |&angle| {
                Point::new_by_radius_angle(radius, angle).translate(&position)
            });

        let inner_points = angles_vec.iter().map(
            |&angle| {
                Point::new_by_radius_angle(radius - 40., angle).translate(&position)
            }).rev();

        let all_points = outer_points.chain(inner_points);

        let vertices: Vec<Vec2d> = all_points
            .map(|p| Vec2d::from(p))
            .collect();


        Polygon::new([0.0, 0.5, 0.0, 1.0])
            .draw(&vertices,
                  &c.draw_state,
                  c.transform,
                  gl);

        for (i, p) in vertices.iter().enumerate() {
            let mut text = graphics::Text::new(24);
            text.color = [1.0, 0.5, 0.0, 1.0];
            text.draw(&format!("{}", i),
            &mut GlyphCache::new(&Path::new("resources/FiraMono-Bold.ttf")).unwrap(),
            &c.draw_state,
            c.trans(p[0], p[1]).transform,
            gl);
        }
    }
}


/// A `Point` represents a position in space
#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    /// Returns a new `Point` with the given coordinates
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }

    pub fn new_by_radius_angle(radius: f64, angle:f64) -> Point {
        Point {
            x: radius * angle.sin(),
            y: radius * angle.cos(),
        }
    }

    /// Translates the point by another point
    pub fn translate(mut self, other: &Point) -> Point {
        self.x += other.x;
        self.y += other.y;
        self
    }
}

impl From<Point> for Vec2d {
    fn from(p: Point) -> Self {
        [p.x, p.y]
    }
}
