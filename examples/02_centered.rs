//! Prints two lines of text to the center of the screen.

use milk_tea::{
    area::Area,
    draw_call::{DrawCall, DrawCallKind},
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    run,
    text_size::UnicodeSize,
};

fn main() {
    run(Model::default(), view, update).unwrap();
}

fn view(_model: &Model, area: &mut Area) {
    let text_0 = "this text is centered!".limit_size(area.size());
    let text_1 = "try resizing the window ^.^".limit_size(area.size());

    // `center_size` returns a position in the center of the `Area` according to a passed in size.
    let pos_0 = area.center_rect(text_0.rect()).pos;
    let pos_1 = area.center_rect(text_1.rect()).pos.map_y(|y| y + 1);

    area.push_all(vec![
        DrawCall::new(pos_0, DrawCallKind::PrintLine(text_0)),
        DrawCall::new(pos_1, DrawCallKind::PrintLine(text_1)),
    ]);
}

fn update(event: Event, model: &mut Model) {
    if let Event::Key(KeyEvent {
        code: KeyCode::Esc,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        model.0 = true;
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
struct Model(bool);

impl milk_tea::Model for Model {
    fn should_exit(&self) -> bool {
        self.0
    }
}
