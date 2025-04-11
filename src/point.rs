use std::ops::{Add, AddAssign, Mul, Sub};
use num::{self, traits::NumAssign, Float, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point2d<T: NumAssign> {
    pub x: T,
    pub y: T,
}

impl<T: NumAssign + Copy> Zero for Point2d<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn set_zero(&mut self) {
        self.x.set_zero();
        self.y.set_zero();
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
}

impl<T: NumAssign + Copy> Add for Point2d<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: NumAssign + Copy> Sub for Point2d<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: NumAssign + Copy> AddAssign for Point2d<T> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T: NumAssign + Copy> Mul<T> for Point2d<T> {
    type Output = Self;

    fn mul(self, multiplier: T) -> Self {
        Self::new(self.x * multiplier, self.y * multiplier)
    }
}

impl<T: NumAssign + Copy> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> T
    where
        T: Float,
    {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn rotate_clockwise(&mut self, angle: T) -> Self
    where
        T: Float,
    {
        let cos = angle.cos();
        let sin = angle.sin();

        let x = self.x * cos + self.y * sin;
        let y = self.x * -sin + self.y * cos;
        Point2d::new(x, y)
    }

    pub fn rotate(&mut self, angle: T) -> Self
    where
        T: Float,
    {
        let rotated = self.rotate_clockwise(angle);
        Point2d::new(rotated.x, rotated.y)
    }
    pub fn normalize(&self) -> Self

    where
        T: Float,
    {
        let zero_point = Self::zero();
        let length = self.distance(&zero_point);
        if length == T::zero() {
            return Point2d::<T>::zero();
        }
        Point2d::new(self.x / length, self.y / length)
    }

    pub fn round(&self) -> Self
    where
        T: Float,
    {
        Point2d::new(self.x.round(), self.y.round())
    }
}

impl Point2d<f64> {
    pub fn to_u16(self) -> Point2d<u16> {
        Point2d::new(self.x as u16, self.y as u16)
    }

    pub fn to_i16(self) -> Point2d<i16> {
        Point2d::new(self.x as i16, self.y as i16)
    }
}
