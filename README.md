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

use milk_tea::{
    area::Area,
    draw_call::{DrawCall, DrawCallKind},
    event::{Event, KeyCode, KeyEvent},
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
            (0, 0).into(),
            DrawCallKind::SetStyle(ContentStyle::new().magenta().bold()),
        ),
        DrawCall::new(
            (0, 0).into(),
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
```

More examples can be found in the `examples` directory. Use `cargo run --example example_name` to run an example.

