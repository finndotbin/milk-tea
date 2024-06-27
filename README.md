# 🧋 Milk Tea

Milk Tea is a minimal Rust library for creating TUI (terminal user interface) apps. It takes a functional approach to constructing applications inspired by many modern web frameworks.

[Bubble Tea]: https://github.com/charmbracelet/bubbletea

**NOTE:** This library has nothing to do with the Go library [Bubble Tea]. It just happens to also be named around boba tea and just happens to also be an MVU-based TUI library. (I don't know how to Google project names apparently QwQ)

## Getting Started

```sh
cargo add milk-tea
```


```rust
//! Prints "hello world :3" in magenta and bold to the top left of the screen.

use crossterm::{
    event::{Event, KeyCode, KeyEvent},
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
        area.push_all(vec![
            DrawCall::new(
                (0, 0).into(),
                DrawCallKind::SetStyle(ContentStyle::new().magenta().bold()),
            ),
            DrawCall::new(
                (0, 0).into(),
                DrawCallKind::PrintLine("hello world! :3".limit_size(area.size())),
            ),
        ]);
    }
}
```

More examples can be found in the `examples` directory. Use `cargo run --example example_name` to run an example.

