//! Prints "hello world :3" in magenta and bold to the top left of the screen.

use milk_tea::{
    area::Area,
    draw_call::{DrawCall, DrawCallKind},
    event::{Event, KeyCode, KeyEvent},
    pair::Pair,
    run,
    style::{ContentStyle, Stylize},
    text_size::UnicodeSize,
};

fn main() {
    run(Model::default(), view, update).unwrap();
}

/// Handles drawing to the screen. An `Area` collects any draw calls we push to it to be rendered
/// later.
fn view(_model: &Model, area: &mut Area) {
    area.push_all(vec![
        DrawCall::new(
            Pair::fill(0),
            DrawCallKind::SetStyle(ContentStyle::new().magenta().bold()),
        ),
        DrawCall::new(
            Pair::fill(0),
            DrawCallKind::PrintLine("hello world! :3".limit_size(area.size())),
        ),
    ]);
}

/// Handles events and updates the `Model`.
fn update(event: Event, model: &mut Model) {
    if let Event::Key(KeyEvent {
        code: KeyCode::Esc, ..
    }) = event
    {
        model.0 = true
    }
}

/// Represents the application state.
#[derive(Default, Clone, PartialEq, Eq)]
struct Model(bool);

impl milk_tea::Model for Model {
    fn should_exit(&self) -> bool {
        self.0
    }
}
