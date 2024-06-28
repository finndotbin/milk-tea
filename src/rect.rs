use crate::pair::{Pair, Pos, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub pos: Pair<Pos>,
    pub size: Pair<Size>,
}

impl Rect {
    pub fn new(pos: Pair<Pos>, size: Pair<Size>) -> Self {
        Self { pos, size }
    }

    pub fn from_size(size: Pair<Size>) -> Self {
        Self {
            pos: Pair::fill(0),
            size,
        }
    }

    pub fn with_pos(&self, pos: Pair<Pos>) -> Self {
        Self {
            pos,
            size: self.size,
        }
    }

    pub fn with_size(&self, size: Pair<Size>) -> Self {
        Self {
            pos: self.pos,
            size,
        }
    }

    pub fn map_pos(&self, f: impl Fn(Pair<Pos>) -> Pair<Pos>) -> Self {
        Self {
            pos: f(self.pos),
            size: self.size,
        }
    }

    pub fn map_size(&self, f: impl Fn(Pair<Size>) -> Pair<Size>) -> Self {
        Self {
            pos: self.pos,
            size: f(self.size),
        }
    }

    pub fn pad(&self, padding: u16) -> Self {
        self.pad_x(padding).pad_y(padding)
    }

    pub fn pad_x(&self, padding: u16) -> Self {
        self.pad_left(padding).pad_right(padding)
    }

    pub fn pad_y(&self, padding: u16) -> Self {
        self.pad_top(padding).pad_bottom(padding)
    }

    pub fn pad_left(&self, padding: u16) -> Self {
        self.map_pos(|pos| pos.map_x(|x| x + padding))
    }

    pub fn pad_right(&self, padding: u16) -> Self {
        self.map_size(|size| size.map_x(|x| x - padding))
    }

    pub fn pad_top(&self, padding: u16) -> Self {
        self.map_pos(|pos| pos.map_y(|y| y + padding))
    }

    pub fn pad_bottom(&self, padding: u16) -> Self {
        self.map_size(|size| size.map_y(|y| y - padding))
    }

    pub fn center_in(&self, outer: Self) -> Self {
        self.with_pos(outer.center() - self.center())
    }

    pub fn center(&self) -> Pair<Pos> {
        self.pos + self.size.map(|xy| xy / 2).into()
    }

    pub fn corner(&self) -> Pair<Pos> {
        self.pos + self.size.into() - Pair::fill(1)
    }

    pub fn is_inside(&self, outer: Self) -> bool {
        let corner = self.corner();
        let outer_corner = outer.corner();

        self.pos.x >= outer.pos.x
            && self.pos.y >= outer.pos.y
            && corner.x <= outer_corner.x
            && corner.y <= outer_corner.y
    }
}
