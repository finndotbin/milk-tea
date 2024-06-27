//! Displays a counter than can be incremented and decremented with `k` and `j`.

use std::cmp::Ordering;

use milk_tea::{
    area::{Area, Element},
    draw_call::{DrawCall, DrawCallKind},
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    pair::Pair,
    run,
    style::{ContentStyle, Stylize},
    text_size::UnicodeSize,
};

fn main() {
    run(Model::default(), view, update).unwrap();
}

fn view(state: &Model, area: &mut Area) {
    let style = match state.count.cmp(&0) {
        Ordering::Greater => ContentStyle::new().cyan(),
        Ordering::Less => ContentStyle::new().magenta(),
        Ordering::Equal => ContentStyle::new().grey(),
    };

    area.sub_element(
        Pair::fill(0),
        area.size(),
        center(style, format!("{}", state.count)),
    )
    .unwrap();
}

/// Returns an `Element` with centered text. `Element`s are just closures that take in an `&mut
/// Area` to push draw calls to.
fn center(style: ContentStyle, text: String) -> Element {
    Box::new(move |area| {
        area.push_all(vec![
            DrawCall::new(area.center_size(text.size()), DrawCallKind::SetStyle(style)),
            DrawCall::new(
                area.center_size(text.size()),
                DrawCallKind::PrintLine(text.limit_size(area.size())),
            ),
        ]);
    })
}

fn update(event: Event, model: &mut Model) {
    if let Event::Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        // Update the count based on the key pressed.
        match code {
            KeyCode::Esc => model.should_exit = true,
            KeyCode::Char('k') => model.count += 1,
            KeyCode::Char('j') => model.count -= 1,
            _ => {}
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
struct Model {
    should_exit: bool,
    count: i32,
}

impl milk_tea::Model for Model {
    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
