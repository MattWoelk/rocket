use graphics;
use opengl_graphics::GlGraphics;
use rand::Rng;

use drawing::Size;
use models::{Bullet, Wave, Enemy, Particle, Player};
use models::CollisionTestBall;
use traits::Entity;
use rand::{self, ThreadRng};
use maths::Point;

use std::iter::Iterator;

/// A model that contains the other models and renders them
pub struct Level_0 {
    pub score: u32,
    pub actions: Actions,
    pub timers: Timers,
    pub rng: ThreadRng,
}

pub enum Controls {
    A(bool),
    B(bool),
    X(bool),
    Y(bool),
    LT(bool),
    RT(bool),
    LB(bool),
    RB(bool),
    X1(i64),
    Y1(i64),
    X2(i64),
    Y2(i64),
    None,
}

#[derive(Default)]
pub struct Actions {
    pub player_velocity: Point,
    pub boost: bool,
    pub shoot: bool,
    pub grass: bool,
    pub water: bool,
    pub fire: bool,
}

/// Timers to handle creation of enemies and particles
pub struct Timers {
    pub current_time: f64,
    pub last_tail_particle: f64,
    pub last_shoot: f64,
    pub last_spawned_enemy: f64
}

impl Timers {
    fn new() -> Self {
        Timers {
            current_time: 0.,
            last_tail_particle: 0.,
            last_shoot: 0.,
            last_spawned_enemy: 0.,
        }
    }
}

impl Level_0 {
    /// Returns a new level of the given size
    pub fn new() -> Level_0 {
        let mut rng = rand::thread_rng();

        Level_0 {
            score: 0,
            actions: Actions::default(),
            timers: Timers::new(),
            rng: rng,
        }
    }

    pub fn handle_key(&mut self, control: Controls) {
        match control {
            Controls::X1(val) => self.actions.player_velocity.x = val as f64,
            Controls::Y1(val) => self.actions.player_velocity.y = val as f64,
            Controls::X(pressed) => self.actions.shoot = pressed,
            _ => ()
        }
    }
}
