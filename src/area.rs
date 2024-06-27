use crate::{
    draw_call::{DrawCall, DrawCalls, NonAbsolute},
    pair::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    pos: Pair<Pos>,
    size: Pair<Size>,
    calls: DrawCalls,
}

impl Area {
    pub fn new(pos: Pair<Pos>, size: Pair<Size>) -> Self {
        Self {
            pos,
            size,
            calls: DrawCalls::new(),
        }
    }

    pub fn size(&self) -> Pair<Size> {
        self.size
    }

    pub fn center(&self) -> Pair<Pos> {
        self.size.center()
    }

    pub fn center_size(&self, size: Pair<Size>) -> Pair<Pos> {
        self.size.center().sub(size.center())
    }

    pub fn push(&mut self, call: DrawCall<NonAbsolute>) {
        let _ = self.try_push(call);
    }

    pub fn push_all(&mut self, calls: Vec<DrawCall<NonAbsolute>>) {
        let _ = self.try_push_all(calls);
    }

    pub fn try_push(&mut self, call: DrawCall<NonAbsolute>) -> Result<(), OutOfAreaError> {
        if let Some(absolute) = call.to_absolute(self.pos, self.size) {
            self.calls.push(absolute);

            return Ok(());
        }

        Err(OutOfAreaError)
    }

    pub fn try_push_all(
        &mut self,
        calls: Vec<DrawCall<NonAbsolute>>,
    ) -> Result<(), OutOfAreaError> {
        for call in calls {
            self.try_push(call)?;
        }

        Ok(())
    }

    pub fn sub_element(
        &mut self,
        sub_pos: Pair<Pos>,
        sub_size: Pair<Size>,
        sub_element: Box<dyn Element>,
    ) -> Result<(), OutOfAreaError> {
        let sub_pos = self.pos.add(sub_pos);

        if !sub_pos.is_inside(sub_size, self.pos, self.size) {
            return Err(OutOfAreaError);
        }

        let mut sub_area = Area::new(sub_pos, sub_size);
        sub_element.draw(&mut sub_area);

        self.calls.extend(sub_area.collect());

        Ok(())
    }

    pub(crate) fn collect(self) -> DrawCalls {
        self.calls
    }
}

#[derive(Debug)]
pub struct OutOfAreaError;

pub trait Element {
    fn draw(&self, area: &mut Area);
}
