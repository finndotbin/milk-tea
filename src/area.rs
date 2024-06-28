use crate::{
    draw_call::{DrawCall, DrawCalls, NonAbsolute},
    pair::*,
    rect::Rect,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    rect: Rect,
    calls: DrawCalls,
}

impl Area {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            calls: DrawCalls::new(),
        }
    }

    pub fn rect(&self) -> Rect {
        self.rect.with_pos(Pair::fill(0))
    }

    pub fn size(&self) -> Pair<Size> {
        self.rect.size
    }

    pub fn center(&self) -> Pair<Pos> {
        self.rect.with_pos(Pair::fill(0)).center()
    }

    pub fn center_rect(&self, rect: Rect) -> Rect {
        rect.center_in(self.rect())
    }

    pub fn push(&mut self, call: DrawCall<NonAbsolute>) {
        let _ = self.try_push(call);
    }

    pub fn push_all(&mut self, calls: Vec<DrawCall<NonAbsolute>>) {
        let _ = self.try_push_all(calls);
    }

    pub fn try_push(&mut self, call: DrawCall<NonAbsolute>) -> Result<(), AreaOverflowError> {
        if let Some(absolute) = call.to_absolute(self.rect) {
            self.calls.push(absolute);

            return Ok(());
        }

        Err(AreaOverflowError)
    }

    pub fn try_push_all(
        &mut self,
        calls: Vec<DrawCall<NonAbsolute>>,
    ) -> Result<(), AreaOverflowError> {
        for call in calls {
            self.try_push(call)?;
        }

        Ok(())
    }

    pub fn push_element(&mut self, rect: Rect, element: Element) {
        let _ = self.try_push_element(rect, element);
    }

    pub fn try_push_element(
        &mut self,
        rect: Rect,
        element: Element,
    ) -> Result<(), AreaOverflowError> {
        let mut area = Area::new(rect.map_pos(|pos| pos + self.rect.pos));

        if !area.rect.is_inside(self.rect) {
            return Err(AreaOverflowError);
        }

        element(&mut area);

        self.calls.extend(area.collect());

        Ok(())
    }

    pub(crate) fn collect(self) -> DrawCalls {
        self.calls
    }
}

#[derive(Debug)]
pub struct AreaOverflowError;

pub type Element = Box<dyn Fn(&mut Area)>;
