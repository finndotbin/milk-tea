//! Uses elements to divide the screen into segments.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use milk_tea::{
    area::{Area, Element},
    draw_call::{DrawCall, DrawCallKind},
    rect::Rect,
    run,
    text_size::UnicodeSize,
};

fn main() {
    run(Model::default(), view, update).unwrap();
}

fn view(_model: &Model, area: &mut Area) {
    // Split the screen into three sections, one upper and two lower.
    let upper = area.size().map_y(|y| y / 2).as_rect();

    let lower_size = area.size().map(|xy| xy / 2);
    let lower_left = Rect::new(lower_size.with_x(0).into(), lower_size);
    let lower_right = Rect::new(lower_size.into(), lower_size);

    area.push_element(upper, Box::new(center("top text".to_owned())));
    area.push_element(lower_left, Box::new(center("lower left text".to_owned())));
    area.push_element(lower_right, Box::new(center("lower right text".to_owned())));
}

/// Returns an `Element` with centered text. `Element`s are just closures that take in an `&mut
/// Area` to push draw calls to.
fn center(text: String) -> Element {
    Box::new(move |area| {
        area.push_all(vec![DrawCall::new(
            area.center_rect(text.rect()).pos,
            DrawCallKind::PrintLine(text.limit_size(area.size())),
        )]);
    })
}

fn update(event: Event, app: &mut Model) {
    if let Event::Key(KeyEvent {
        code: KeyCode::Esc,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        app.0 = true;
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
struct Model(bool);

impl milk_tea::Model for Model {
    fn should_exit(&self) -> bool {
        self.0
    }
}
