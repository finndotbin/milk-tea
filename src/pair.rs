use std::{marker::PhantomData, ops};

use crate::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pair<T> {
    pub x: u16,
    pub y: u16,
    _kind: PhantomData<T>,
}

impl<T> Pair<T> {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            _kind: PhantomData,
        }
    }

    pub fn fill(x: u16) -> Self {
        Self::new(x, x)
    }

    pub fn with_x(&self, x: u16) -> Self {
        self.map_x(|_| x)
    }

    pub fn with_y(&self, y: u16) -> Self {
        self.map_y(|_| y)
    }

    pub fn map(&self, f: impl Fn(u16) -> u16) -> Self {
        self.map_x(&f).map_y(&f)
    }

    pub fn map_x(&self, f: impl Fn(u16) -> u16) -> Self {
        Self::new(f(self.x), self.y)
    }

    pub fn map_y(&self, f: impl Fn(u16) -> u16) -> Self {
        Self::new(self.x, f(self.y))
    }

    pub fn combine(&self, pair: Self, f: impl Fn(u16, u16) -> u16) -> Self {
        Self::new(f(self.x, pair.x), f(self.y, pair.y))
    }
}

impl<T> From<(u16, u16)> for Pair<T> {
    fn from(value: (u16, u16)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> ops::Add for Pair<T> {
    type Output = Self;

    fn add(self, pair: Pair<T>) -> Self {
        self.combine(pair, |a, b| a + b)
    }
}

impl<T> ops::Sub for Pair<T> {
    type Output = Self;

    fn sub(self, pair: Pair<T>) -> Self {
        self.combine(pair, |a, b| a - b)
    }
}

impl From<Pair<Size>> for Pair<Pos> {
    fn from(value: Pair<Size>) -> Self {
        Pair::new(value.x, value.y)
    }
}

impl Pair<Size> {
    pub fn as_rect(&self) -> Rect {
        Rect::from_size(*self)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size;
