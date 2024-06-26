use crate::pair::{Pair, Size};
use unicode_display_width::width;

pub trait UnicodeSize {
    fn size(&self) -> Pair<Size>;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

impl<T> UnicodeSize for T
where
    T: AsRef<str>,
{
    fn width(&self) -> u16 {
        width(self.as_ref()) as u16
    }

    fn height(&self) -> u16 {
        self.as_ref().lines().count() as u16
    }

    fn size(&self) -> Pair<Size> {
        Pair::new(self.width(), self.height())
    }
}
