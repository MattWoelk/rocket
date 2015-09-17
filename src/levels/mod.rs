
mod level_0;
pub use self::level_0::Level0;

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
