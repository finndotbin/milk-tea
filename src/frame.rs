use crate::{
    draw_call::{single_line, Absolute, DrawCall, DrawCallKind, DrawCalls},
    pair::{Pair, Pos, Size},
    text_size::UnicodeSize,
};
use crossterm::{
    cursor::MoveTo,
    style::{ContentStyle, Print, ResetColor, SetStyle},
    QueueableCommand,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    marker::PhantomData,
};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Frame<IsDiff> {
    styles: HashMap<Pair<Pos>, ContentStyle>,
    graphemes: HashMap<Pair<Pos>, String>,
    _is_diff: PhantomData<IsDiff>,
}

impl Frame<NonDiff> {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            graphemes: HashMap::new(),
            _is_diff: PhantomData,
        }
    }

    pub fn from_calls(calls: &DrawCalls) -> Self {
        let mut frame = Self::new();
        frame.apply_calls(calls);

        frame
    }

    pub fn apply_calls(&mut self, calls: &DrawCalls) {
        for call in calls {
            self.apply_call(call);
        }
    }

    pub fn apply_call(&mut self, call: &DrawCall<Absolute>) {
        match call.kind() {
            DrawCallKind::PrintLine(string) => self.apply_print_line(call.pos(), string),
            DrawCallKind::SetStyle(style) => self.apply_set_style(call.pos(), style),
        }
    }

    fn apply_print_line(&mut self, mut pos: Pair<Pos>, string: &str) {
        let line = single_line(string);

        for grapheme in line.graphemes(true) {
            self.graphemes.insert(pos, grapheme.to_owned());

            pos = pos + Pair::new(grapheme.width(), 0);
        }
    }

    fn apply_set_style(&mut self, pos: Pair<Pos>, style: &ContentStyle) {
        self.styles.insert(pos, *style);
    }

    pub fn diff(&self, was_resized: bool, last: &Frame<NonDiff>) -> Frame<Diff> {
        let graphemes = diff_graphemes(&self.graphemes, &last.graphemes, was_resized);

        Frame {
            styles: self.styles.clone(),
            graphemes,
            _is_diff: PhantomData,
        }
    }
}

impl Frame<Diff> {
    pub fn draw(
        &self,
        was_resized: bool,
        size: Pair<Size>,
        stdout: &mut io::Stdout,
    ) -> io::Result<()> {
        stdout.queue(ResetColor)?;

        for x in 0..size.x {
            for y in 0..size.y {
                let pos = Pair::new(x, y);
                stdout.queue(MoveTo(pos.x, pos.y))?;

                if let Some(style) = self.styles.get(&pos) {
                    stdout.queue(SetStyle(*style))?;
                }

                if let Some(grapheme) = self.graphemes.get(&pos) {
                    stdout.queue(Print(grapheme))?;
                } else if was_resized {
                    stdout.queue(Print(" ".to_owned()))?;
                }
            }
        }

        stdout.flush()?;

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NonDiff;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Diff;

fn diff_graphemes(
    new: &HashMap<Pair<Pos>, String>,
    old: &HashMap<Pair<Pos>, String>,
    force_new: bool,
) -> HashMap<Pair<Pos>, String> {
    let mut old = old.clone();
    let mut diff = HashMap::new();

    for (pos, new_entry) in new.iter() {
        let Some(old_entry) = old.remove(pos) else {
            diff.insert(*pos, new_entry.clone());
            continue;
        };

        if new_entry != &old_entry || force_new {
            diff.insert(*pos, new_entry.clone());
        }
    }

    for (pos, _) in old.iter() {
        diff.insert(*pos, " ".to_owned());
    }

    diff
}
