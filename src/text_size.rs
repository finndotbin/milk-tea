use crate::pair::{Pair, Size};
use unicode_display_width::width;
use unicode_segmentation::UnicodeSegmentation;

pub trait UnicodeSize {
    fn size(&self) -> Pair<Size>;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn limit_size(&self, limit: Pair<Size>) -> String;
    fn limit_width(&self, limit: u16) -> String;
    fn limit_height(&self, limit: u16) -> String;
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

    fn limit_size(&self, limit: Pair<Size>) -> String {
        self.limit_width(limit.x).limit_height(limit.y)
    }

    fn limit_width(&self, limit: u16) -> String {
        let mut width = 0;
        let mut result = String::new();

        for grapheme in self.as_ref().graphemes(true) {
            let next_width = width + grapheme.width() as u16;

            if next_width > limit {
                return result;
            }

            width = next_width;
            result.push_str(grapheme);
        }

        result
    }

    fn limit_height(&self, limit: u16) -> String {
        self.as_ref().lines().take(limit as usize).collect()
    }
}
