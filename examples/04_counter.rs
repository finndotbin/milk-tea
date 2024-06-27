//! Displays a counter than can be incremented and decremented with `k` and `j`.

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    style::{ContentStyle, Stylize},
};
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

fn draw(state: &App) -> Box<dyn Element> {
    let style = if state.count > 0 {
        ContentStyle::new().cyan()
    } else if state.count < 0 {
        ContentStyle::new().magenta()
    } else {
        ContentStyle::new().grey()
    };

    Box::new(Center(style, format!("{}", state.count)))
}

fn update(event: Event, app: &mut App) {
    if let Event::Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        match code {
            KeyCode::Esc => app.should_exit = true,
            KeyCode::Char('k') => app.count += 1,
            KeyCode::Char('j') => app.count -= 1,
            _ => {}
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
struct App {
    should_exit: bool,
    count: i32,
}

impl State for App {
    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

struct Center(ContentStyle, String);

impl Element for Center {
    fn draw(&self, area: &mut Area) {
        let pos = area.center_size(self.1.size());

        area.push_all(vec![
            DrawCall::new(pos, DrawCallKind::SetStyle(self.0)),
            DrawCall::new(pos, DrawCallKind::PrintLine(self.1.limit_size(area.size()))),
        ]);
    }
}
