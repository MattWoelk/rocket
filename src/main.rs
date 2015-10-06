extern crate glutin_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod drawing;
mod game;
mod models;
mod levels;
mod traits;
mod maths;

use glutin_window::GlutinWindow;
use models::{Player};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Event, RenderEvent};
use piston::event_loop::{Events, EventLoop};
use piston::input::{Button, Input};
use piston::input::Key;
use piston::window::WindowSettings;

use drawing::Size;
use game::Game;
use traits::Level;
use levels::Level0;
use levels::CollisionLevel;

use maths::Point;

extern crate sdl2;
use sdl2::event::Event as SDL2Event;
use sdl2::{joystick, controller};
use sdl2::controller::GameController;

use std::env::args;


fn main() {
    // Get command line args
    let arguments = args().collect::<Vec<String>>();
    let size = Size::new(1024.0, 600.0);

    let level: Box<Level> = match arguments.get(1).map(|x| x.as_ref()) {
            Some("l0") => Box::new(Level0::new()),
            Some("collision") => Box::new(Level0::new()),
            _          => {
                println!("WARNING: Level name not found. Using 0 instead.");
                Box::new(Level0::new())
            },
    };

    // Initialization stuff

    let mut sdl_context = sdl2::init().game_controller().unwrap();
    let available =
        match joystick::num_joysticks() {
            Ok(n)  => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

    println!("{} joysticks available", available);

    let mut controller = None;

    // Iterate over all available joysticks and look for game
    // controllers.
    for id in 0..available {
        if controller::is_game_controller(id) {
            println!("Attempting to open controller {}", id);

            match GameController::open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    println!("Controller mapping: {}", c.mapping());
                    controller = Some(c);
                    break;
                },
                Err(e) => println!("failed: {:?}", e),
            }

        } else {
             println!("{} is not a game controller", id);
        }
    }

    if let None = controller {
        println!("Couldn't open any controller");
    }

    let opengl = OpenGL::V3_2;

    let window: GlutinWindow =
        WindowSettings::new("Rocket", [1024, 600])
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    let mut gl = GlGraphics::new(opengl);

    // The game object
    //let l: Box<Level> = Box::new(*level);
    //let mut game = Game::new(
    //    size,
    //    level);

    let mut rng = rand::thread_rng();
    let mut game = Game {
        level: level,
        player: Player::random(&mut rng, size.clone()),
        //particles: vec![],
        //bullets: vec![],
        //waves: vec![],
        entities: vec![],
        //collision_test_balls: vec![],
        size: size.clone(),
        //resources: Resources { font: GlyphCache::new(&Path::new("resources/FiraMono-Bold.ttf")).unwrap() }
    };

    // Event handling
    for e in window.events().ups(60).max_fps(60) {
        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                game.key_press(key);
                if let Key::P = key {
                    println!("P");
                    game.spawn_circle_with_collision_colouring(
                        Point::new(55., 55.),
                    );
                }
            }

            Event::Input(Input::Release(Button::Keyboard(key))) => {
                game.key_release(key);
            }

            Event::Render(args) => {
                gl.draw(args.viewport(), |c, g| game.render(c, g));
            }

            Event::Update(args) => {
                for sdl2_event in sdl_context.event_pump().poll_iter() {
                    match sdl2_event {
                        SDL2Event::ControllerAxisMotion{ axis, value: val, .. } => {
                            game.handle_axis(axis, val as i32);
                        }
                        SDL2Event::ControllerButtonDown{ button, .. } => {
                            game.button_press(button);
                        },
                        SDL2Event::ControllerButtonUp{ button, .. } => {
                            game.button_release(button);
                        }
                        SDL2Event::Quit{..} => break,
                        _ => (),
                    }
                }
                game.update(args.dt);
            }

            _ => {}
        }
    }
}
