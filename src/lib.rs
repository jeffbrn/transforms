mod scratch;
pub mod twod;

pub use scratch::*;

pub trait FrameOfReference {
    fn name() -> &'static str;
}
