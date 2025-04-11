use num::traits::NumAssign;
use crate::point::Point2d;
use std::ops::Range;
use rand::{
    distributions::{
        uniform::SampleUniform,
        Distribution,
        Standard
    },
    rngs::ThreadRng,
    Rng,
};

pub trait Position<T: NumAssign + Copy> {
    fn position(&self) -> Point2d<T>;
    fn set_position(&mut self, position: Point2d<T>);
    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let new_position = Point2d::new(
            rng.gen_range(x_range),
            rng.gen_range(y_range)
        );
        self.set_position(new_position);
    }
}