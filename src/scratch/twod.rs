use std::{marker::PhantomData, ops::Add};

pub trait FrameOfReference {
	fn name() -> &'static str;
}

pub struct Motion<TFromPose, TToPose> where TFromPose: FrameOfReference, TToPose: FrameOfReference {
	_marker: PhantomData<(TFromPose, TToPose)>,
}

impl<TFromPose, TToPose> Motion<TFromPose, TToPose> where TFromPose: FrameOfReference, TToPose: FrameOfReference {
	pub fn new() -> Self {
		Self {
			_marker: PhantomData,
		}
	}
}

impl<TP1,TP2,TP3> Add<Motion<TP2,TP3>> for Motion<TP1,TP2> where TP1: FrameOfReference, TP2: FrameOfReference, TP3: FrameOfReference {
	type Output = Motion<TP1,TP3>;

	fn add(self, _rhs: Motion<TP2,TP3>) -> Self::Output {
		println!("Moving motion from {} to {} via {}", TP1::name(), TP3::name(), TP2::name());
		Motion::new()
	}
}
