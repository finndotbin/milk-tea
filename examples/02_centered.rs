//! Prints two lines of text to the center of the screen.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use milk_tea::{
    area::{Area, Element},
    draw_call::{DrawCall, DrawCallKind},
    run,
    text_size::UnicodeSize,
    State,
};

fn main() {
    run(App::default(), draw, update).unwrap();
}

fn draw(_state: &App) -> Box<dyn Element> {
    Box::new(Center)
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

struct Center;

impl Element for Center {
    fn draw(&self, area: &mut Area) {
        let text_0 = "this text is centered!".to_owned();
        let text_1 = "try resizing the window ^.^".to_owned();

        let pos_0 = area.center_size(text_0.size());
        let pos_1 = area.center_size(text_1.size()).map_y(|y| y + 1);

        area.push_all(vec![
            DrawCall::new(pos_0, DrawCallKind::PrintLine(text_0)),
            DrawCall::new(pos_1, DrawCallKind::PrintLine(text_1)),
        ])
        .unwrap();
    }
}
