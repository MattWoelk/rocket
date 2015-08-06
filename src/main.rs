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
use graphics::{Transformed};
use opengl_graphics::glyph_cache::GlyphCache;

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
    let vertices = [
        [433.0127018922193, 550.0],
        [475.52825814757676, 454.5084971874737],
        [497.2609476841366, 352.26423163382685],
        [497.2609476841367, 247.73576836617332],
        [475.5282581475768, 145.49150281252633],
        [433.01270189221935, 50.000000000000114],
        [398.3716857408418, 70.00000000000011],
        [437.4859974957707, 157.85218258752423],
        [457.4800718694058, 251.91690689687948],
        [457.4800718694057, 348.0830931031207],
        [437.4859974957706, 442.14781741247583],
        [398.37168574084177, 530.0]
    ];

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
