pub mod area;
pub mod draw_call;
pub mod pair;
pub mod text_size;

mod frame;

pub use crossterm::{self, event, style};

use area::Area;
use crossterm::{cursor, event::Event, terminal, ExecutableCommand, QueueableCommand};
use frame::Frame;
use pair::Pair;
use std::io;

pub fn run<M, V, U>(mut model: M, view: V, update: U) -> io::Result<()>
where
    M: Model,
    V: Fn(&M, &mut Area),
    U: Fn(Event, &mut M),
{
    let mut stdout = io::stdout();

    stdout.queue(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut last_state = model.clone();
    let mut last_frame = Frame::new();
    let mut was_resized = true;

    while !model.should_exit() {
        if model != last_state || was_resized {
            let size = terminal::size()?.into();

            let mut area = Area::new(Pair::new(0, 0), size);
            view(&model, &mut area);

            let calls = area.collect();

            let frame = Frame::from_calls(&calls);
            frame
                .diff(was_resized, &last_frame)
                .draw(was_resized, size, &mut stdout)?;

            last_frame = frame;
        }

        last_state = model.clone();

        let event = event::read()?;

        if let Event::Resize(_, _) = event {
            was_resized = true;
        }

        update(event, &mut model);
    }

    stdout.queue(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

pub trait Model: Clone + PartialEq + Eq {
    fn should_exit(&self) -> bool;
}
