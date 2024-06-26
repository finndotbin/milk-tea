use crate::{
    pair::{Pair, Pos, Size},
    text_size::UnicodeSize,
};
use crossterm::style::ContentStyle;
use std::marker::PhantomData;

pub type DrawCalls = Vec<DrawCall<Absolute>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCall<IsAbsolute> {
    pos: Pair<Pos>,
    kind: DrawCallKind,
    _is_absolute: PhantomData<IsAbsolute>,
}

impl<T> DrawCall<T> {
    pub fn new(pos: Pair<Pos>, kind: DrawCallKind) -> Self {
        Self {
            pos,
            kind,
            _is_absolute: PhantomData,
        }
    }

    pub fn pos(&self) -> Pair<Pos> {
        self.pos
    }

    pub fn kind(&self) -> &DrawCallKind {
        &self.kind
    }

    pub fn size(&self) -> Pair<Size> {
        match &self.kind {
            DrawCallKind::PrintLine(string) => Pair::new(single_line(string).width(), 1),
            DrawCallKind::SetStyle(_) => Pair::new(1, 1),
        }
    }
}

impl DrawCall<NonAbsolute> {
    pub fn to_absolute(&self, pos: Pair<Pos>, size: Pair<Size>) -> Option<DrawCall<Absolute>> {
        let self_pos = self.pos.add(pos);

        if !self_pos.is_inside(self.size(), pos, size) {
            return None;
        }

        Some(DrawCall {
            pos: self_pos,
            kind: self.kind.clone(),
            _is_absolute: PhantomData,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonAbsolute;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Absolute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawCallKind {
    PrintLine(String),
    SetStyle(ContentStyle),
}

pub fn single_line(string: &str) -> String {
    string.lines().collect::<Vec<&str>>().concat()
}
