use crate::FrameOfReference;
use nalgebra::Vector2;
use std::{
    marker::PhantomData,
    ops::{Add, Not, Sub},
};

#[derive(PartialEq, Debug)]
pub struct Vec2<TFrame>
where
    TFrame: FrameOfReference,
{
    v: Vector2<f64>,
    _marker: PhantomData<TFrame>,
}

impl<TFrame> Vec2<TFrame>
where
    TFrame: FrameOfReference,
{
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            v: Vector2::new(x, y),
            _marker: PhantomData,
        }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn as_vector(&self) -> Vector2<f64> {
        self.v
    }
}

impl<TFrame> Not for &Vec2<TFrame>
where
    TFrame: FrameOfReference,
{
    type Output = Vec2<TFrame>;

    fn not(self) -> Self::Output {
        Self::Output {
            v: -self.v,
            _marker: PhantomData,
        }
    }
}

impl<TFrame> Add<&Vec2<TFrame>> for &Vec2<TFrame>
where
    TFrame: FrameOfReference,
{
    type Output = Vec2<TFrame>;

    fn add(self, rhs: &Vec2<TFrame>) -> Self::Output {
        Self::Output {
            v: self.v + rhs.v,
            _marker: PhantomData,
        }
    }
}

impl<TFrame> Sub<&Vec2<TFrame>> for &Vec2<TFrame>
where
    TFrame: FrameOfReference,
{
    type Output = Vec2<TFrame>;

    fn sub(self, rhs: &Vec2<TFrame>) -> Self::Output {
        Self::Output {
            v: self.v - rhs.v,
            _marker: PhantomData,
        }
    }
}
