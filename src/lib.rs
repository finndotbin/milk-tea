pub mod area;
pub mod draw_call;
pub mod pair;
pub mod text_size;

mod frame;

use area::{Area, Element};
use crossterm::{
    cursor,
    event::{self, Event},
    terminal, ExecutableCommand, QueueableCommand,
};
use frame::Frame;
use pair::Pair;
use std::io;

pub fn run<S, V, U>(mut state: S, view: V, update: U) -> io::Result<()>
where
    S: State,
    V: Fn(&S) -> Box<dyn Element>,
    U: Fn(Event, &mut S),
{
    let mut stdout = io::stdout();

    stdout.queue(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let mut last_state = state.clone();
    let mut last_frame = Frame::new();
    let mut element = view(&state);
    let mut was_resized = true;

    while !state.should_exit() {
        if state != last_state || was_resized {
            let size = terminal::size()?.into();

            let calls = {
                let mut area = Area::new(Pair::new(0, 0), size);
                element.draw(&mut area);

                area.collect()
            };

            let frame = Frame::from_calls(&calls);
            if was_resized {
                frame
                    .diff(was_resized, &last_frame)
                    .draw(was_resized, size, &mut stdout)?;
            }

            last_frame = frame;
            element = view(&state);
        }

        last_state = state.clone();

        let event = event::read()?;

        if let Event::Resize(_, _) = event {
            was_resized = true;
        }

        update(event, &mut state);
    }

    stdout.queue(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}

pub trait State: Clone + PartialEq + Eq {
    fn should_exit(&self) -> bool;
}
