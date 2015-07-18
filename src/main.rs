extern crate glutin_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod drawing;
mod game;
mod models;
mod traits;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event::{Event, Events, EventLoop, RenderEvent};
use piston::input::{Button, Input};
use piston::window::WindowSettings;

use drawing::Size;
use game::Game;

extern crate sdl2;
use sdl2::event::Event as SDL2Event;
use sdl2::{joystick, controller};
use sdl2::controller::GameController;


enum AnyEvent {
    Piston(Event),
    Sdl2(SDL2Event),
}

fn main() {
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
                    controller = Some(c);
                    break;
                },
                Err(e) => println!("failed: {:?}", e),
            }

        } else {
             println!("{} is not a game controller", id);
        }
    }

    let controller =
        match controller {
            Some(c) => c,
            None     => panic!("Couldn't open any controller"),
        };

    println!("Controller mapping: {}", controller.mapping());

    for event in sdl_context.event_pump().wait_iter() {
        match event {
            SDL2Event::ControllerAxisMotion{ axis, value: val, .. } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                if (val as i32).abs() > 10000 {
                    println!("Axis {:?} moved to {}", axis, val);
                }
            }
            SDL2Event::ControllerButtonDown{ button, .. } =>
                println!("Button {:?} down", button),
            SDL2Event::ControllerButtonUp{ button, .. } =>
                println!("Button {:?} up", button),
            SDL2Event::Quit{..} => break,
            _ => (),
        }
    }

    let opengl = OpenGL::_3_2;

    let window: GlutinWindow =
        WindowSettings::new("Rocket", [1024, 600])
            .exit_on_esc(true)
            .opengl(opengl)
            .samples(8)
            .into();

    let mut gl = GlGraphics::new(opengl);

    // The game object
    let mut game = Game::new(Size::new(1024.0, 600.0));

    // Event handling
    for any_event in window.events().ups(60).max_fps(60).map(|it| AnyEvent::Piston(it)) {
        match any_event {
            AnyEvent::Piston(e) => {
                match e {
                    Event::Input(Input::Press(Button::Keyboard(key))) => {
                        game.key_press(key);
                    }

                    Event::Input(Input::Release(Button::Keyboard(key))) => {
                        game.key_release(key);
                    }

                    Event::Render(args) => {
                        gl.draw(args.viewport(), |c, g| game.render(c, g));
                    }

                    Event::Update(args) => {
                        // TODO: check controller using something like sdl2
                        game.update(args.dt);
                    }

                    _ => {}
                }
            }

            _ => {println!("NEW")}
        }
    }
}
