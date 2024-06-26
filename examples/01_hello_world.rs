use crossterm::event::{Event, KeyCode, KeyEvent};
use milk_tea::{
    area::{Area, Element},
    draw_call::{DrawCall, DrawCallKind},
    run, State,
};

fn main() {
    run(App::default(), draw, update).unwrap();
}

fn draw(_state: &App) -> Box<dyn Element> {
    Box::new(Hello)
}

fn update(event: Event, state: &mut App) {
    if let Event::Key(KeyEvent {
        code: KeyCode::Esc, ..
    }) = event
    {
        state.0 = true
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
struct App(bool);

impl State for App {
    fn should_exit(&self) -> bool {
        self.0
    }
}

struct Hello;

impl Element for Hello {
    fn draw(&self, area: &mut Area) {
        area.push(DrawCall::new(
            (0, 0).into(),
            DrawCallKind::PrintLine("hello world! :3".to_owned()),
        ))
        .unwrap();
    }
}
