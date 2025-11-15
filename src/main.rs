use transforms_lib::twod::{FrameOfReference, Motion, Position};

struct Cmech {}
struct Oof {}
struct Rso {}

impl FrameOfReference for Cmech {
    fn name() -> &'static str {
        "Cmech"
    }
}

impl FrameOfReference for Oof {
    fn name() -> &'static str {
        "Oof"
    }
}

impl FrameOfReference for Rso {
    fn name() -> &'static str {
        "Rso"
    }
}

fn main() {
    println!("Hello, world!");
    let m1: Motion<Cmech, Oof> = Motion::new();
    let m2: Motion<Oof, Rso> = Motion::new();
    let _m3 = &m1 + &m2;
    let _m4 = Motion::<Cmech, Rso>::new();
    //let m5 = m1 + m4; - Incorrect won't compile

    let pos_oof: Position<Oof> = Position::new(1.0, 2.0);
    let _pos_cmech = &m1 * &pos_oof;
}
