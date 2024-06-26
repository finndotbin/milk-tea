use std::marker::PhantomData;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl Pair<Pos> {
    pub fn offset(self, offset: Pair<Pos>) -> Self {
        Self::new(self.x + offset.x, self.y + offset.y)
    }

    pub fn corner(self, size: Pair<Size>) -> Self {
        self.offset(Self::new(size.x - 1, size.y - 1))
    }

    pub fn is_inside(self, self_size: Pair<Size>, pos: Pair<Pos>, size: Pair<Size>) -> bool {
        let self_corner = self.corner(self_size);
        let corner = pos.corner(size);

        self.x >= pos.x && self.y >= pos.y && self_corner.x <= corner.x && self_corner.y <= corner.y
    }
}

impl<T> From<(u16, u16)> for Pair<T> {
    fn from(value: (u16, u16)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size;
