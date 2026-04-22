use crate::{FrameOfReference, twod::SE2, twod::Vec2};
use std::{
    marker::PhantomData,
    ops::{Mul, Not},
};

#[derive(PartialEq, Debug)]
pub struct Transform<TFrom, TTo>
where
    TFrom: FrameOfReference,
    TTo: FrameOfReference,
{
    t: SE2,
    _marker: PhantomData<(TFrom, TTo)>,
}

impl<TFrom, TTo> Transform<TFrom, TTo>
where
    TFrom: FrameOfReference,
    TTo: FrameOfReference,
{
    pub fn new(t: SE2) -> Self {
        Self {
            t,
            _marker: PhantomData,
        }
    }
    pub fn is_null(&self) -> bool {
        self.t.is_identity()
    }
}

impl<TFrom, TTo> Not for &Transform<TFrom, TTo>
where
    TFrom: FrameOfReference,
    TTo: FrameOfReference,
{
    type Output = Transform<TTo, TFrom>;

    fn not(self) -> Self::Output {
        println!(
            "Inverting transform from {} to {}",
            TFrom::name(),
            TTo::name()
        );
        Self::Output::new(!&self.t)
    }
}

impl<TP1, TP2, TP3> Mul<&Transform<TP2, TP3>> for &Transform<TP1, TP2>
where
    TP1: FrameOfReference,
    TP2: FrameOfReference,
    TP3: FrameOfReference,
{
    type Output = Transform<TP1, TP3>;

    fn mul(self, rhs: &Transform<TP2, TP3>) -> Self::Output {
        println!(
            "Combining transform from {} to {} via {}",
            TP1::name(),
            TP3::name(),
            TP2::name()
        );
        let result = &self.t * &rhs.t;
        println!("  Result: {:?}", result);
        Self::Output::new(result)
    }
}

impl<TFrom, TTo> Mul<&Vec2<TFrom>> for &Transform<TFrom, TTo>
where
    TFrom: FrameOfReference,
    TTo: FrameOfReference,
{
    type Output = Vec2<TTo>;

    fn mul(self, rhs: &Vec2<TFrom>) -> Self::Output {
        let v = &self.t * rhs.as_vector();
        Self::Output::new(v[0], v[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twod::SO2;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::FRAC_PI_2;

    #[derive(Debug)]
    struct A {}
    #[derive(Debug)]
    struct B {}
    #[derive(Debug)]
    struct C {}

    impl FrameOfReference for A {
        fn name() -> &'static str {
            "A"
        }
    }

    impl FrameOfReference for B {
        fn name() -> &'static str {
            "B"
        }
    }

    impl FrameOfReference for C {
        fn name() -> &'static str {
            "C"
        }
    }

    #[test]
    fn test_transform_operations() {
        let so = SO2::new(FRAC_PI_2);
        let t = SE2::new(so, nalgebra::Vector2::new(1.0, 2.0));
        let transform_ab: Transform<A, B> = Transform::new(t);
        let vec_a: Vec2<A> = Vec2::new(3.0, 4.0);
        let vec_b: Vec2<B> = &transform_ab * &vec_a;
        assert!((vec_b.x() - (-3.0)).abs() < 1e-10);
        assert!((vec_b.y() - 5.0).abs() < 1e-10);

        let transform_ba = !&transform_ab;
        let vec_a_recovered: Vec2<A> = &transform_ba * &vec_b;
        assert!((vec_a_recovered.x() - vec_a.x()).abs() < 1e-10);
        assert!((vec_a_recovered.y() - vec_a.y()).abs() < 1e-10);
    }

    #[test]
    fn test_inverse() {
        let so = SO2::new(FRAC_PI_2);
        let t = SE2::new(so, nalgebra::Vector2::new(1.0, 2.0));
        let transform_ab: Transform<A, B> = Transform::new(t);
        let transform_ba = !&transform_ab;
        let identity_ab = &transform_ab * &transform_ba;
        assert!(identity_ab.is_null());
    }
}
