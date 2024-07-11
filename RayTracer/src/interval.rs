use std::{
    f64::{INFINITY, NEG_INFINITY},
    ops::Add,
};

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: &Self, b: &Self) -> Self {
        Self::new(
            if a.min <= b.min { a.min } else { b.min },
            if a.max >= b.max { a.max } else { b.max },
        )
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub const EMPTY: Self = Self::new(INFINITY, NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(NEG_INFINITY, INFINITY);
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}
