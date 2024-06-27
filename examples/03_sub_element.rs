//! Uses sub-elements to divide the screen into segments.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use milk_tea::{
    area::{Area, Element},
    draw_call::{DrawCall, DrawCallKind},
    pair::Pair,
    run,
    text_size::UnicodeSize,
    State,
};

fn main() {
    run(App::default(), draw, update).unwrap();
}

fn draw(_state: &App) -> Box<dyn Element> {
    Box::new(Split(
        "this is on the top!".to_owned(),
        "this is on the left!".to_owned(),
        "this is on the right!".to_owned(),
    ))
}

fn update(event: Event, app: &mut App) {
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
struct App(bool);

impl State for App {
    fn should_exit(&self) -> bool {
        self.0
    }
}

struct Split(String, String, String);

impl Element for Split {
    fn draw(&self, area: &mut Area) {
        let upper_size = area.size().map_y(|y| y / 2);
        let lower_size = area.size().map(|xy| xy / 2);

        // Top element
        area.sub_element(Pair::fill(0), upper_size, Box::new(Center(self.0.clone())))
            .unwrap();

        // Bottom-left element
        area.sub_element(
            lower_size.with_x(0).into(),
            lower_size,
            Box::new(Center(self.1.clone())),
        )
        .unwrap();

        // Bottom-right element
        area.sub_element(
            lower_size.into(),
            lower_size,
            Box::new(Center(self.2.clone())),
        )
        .unwrap();
    }
}

struct Center(String);

impl Element for Center {
    fn draw(&self, area: &mut Area) {
        area.push_all(vec![DrawCall::new(
            area.center_size(self.0.size()),
            DrawCallKind::PrintLine(self.0.clone()),
        )])
        .unwrap();
    }
}
