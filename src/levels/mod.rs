
mod level_0;
mod collision_level;
pub use self::level_0::Level0;
pub use self::collision_level::CollisionLevel;
use maths::Point;

pub enum Controls {
    A(bool),
    B(bool),
    X(bool),
    //Y(bool),
    //LT(bool),
    //RT(bool),
    //LB(bool),
    //RB(bool),
    X1(i64),
    Y1(i64),
    //X2(i64),
    //Y2(i64),
    None,
}

#[derive(Default, Clone)]
pub struct Actions {
    pub player_velocity: Point,
    pub boost: bool,
    pub shoot: bool,
    pub grass: bool,
    pub water: bool,
    pub fire: bool,
}

/// Timers to handle creation of enemies and particles
#[derive(Clone)]
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
