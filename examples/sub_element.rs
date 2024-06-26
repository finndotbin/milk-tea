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

struct Split(String, String);

impl Element for Split {
    fn draw(&self, area: &mut Area) {
        let size = area.size().map_x(|x| x / 2);
        area.sub_element(
            Pair::fill(0),
            area.size().map_x(|x| x / 2),
            Box::new(Center(self.0.clone())),
        )
        .unwrap();
        area.sub_element(
            size.with_y(0).into(),
            size,
            Box::new(Center(self.1.clone())),
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
