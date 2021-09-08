#[macro_use]
mod generated;
// reexport
pub use self::generated::*;

struct Pepperoni<A,B,C,D> {
    a: A,
    b: B,
    c: C,
    d: D
}

kind4!(Pepperoni);