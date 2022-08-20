use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    Result,
    style::{Color, Colors, Print, SetColors},
};

use crate::constants::{CHARS, LineChars};
use crate::traits::Draw;

pub enum LineType {
    Double,
    Single,
}

impl Default for LineType {
    fn default() -> Self {
        LineType::Single
    }
}

/// A square with a position and size.
#[derive(Default)]
pub struct Square {
    pub line_type: LineType,
    pub colors: Option<Colors>,
    pub filled: Option<()>,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub struct SquareBuilder {
    square: Square,
}

impl SquareBuilder {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> SquareBuilder {
        SquareBuilder {
            square: Square {
                x,
                y,
                width,
                height,
                ..Default::default()
            },
        }
    }

    pub fn with_colors(mut self, colors: Colors) -> Self {
        self.square.colors = Some(colors);
        self
    }

    pub fn is_filled(mut self, filled: bool) -> Self {
        if filled {
            self.square.filled = Some(());
        } else {
            self.square.filled = None;
        }
        self
    }

    pub fn with_line_type(mut self, line_type: LineType) -> Self {
        self.square.line_type = line_type;
        self
    }

    pub fn build(self) -> Square {
        self.square
    }
}

impl Draw for Square {
    fn draw(&self) -> Result<()> {
        let chars: LineChars = match self.line_type {
            LineType::Double => CHARS.lines.double,
            LineType::Single => CHARS.lines.single,
        };

        if let Some(colors) = &self.colors {
            execute!(stdout(), SetColors(*colors))?;
        }

        execute!(
            stdout(),
            MoveTo(self.x, self.y),
            Print(chars.top_left),
            Print(chars.horizontal.repeat(usize::from(self.width - 2))),
            Print(chars.top_right),
        )?;

        for i in 1..self.height - 1 {
            execute!(stdout(), MoveTo(self.x, self.y + i), Print(chars.vertical))?;

            if self.filled.is_some() {
                execute!(
                    stdout(),
                    Print(" ".repeat(usize::from(self.width - 2))),
                    Print(chars.vertical),
                )?;
            } else {
                execute!(
                    stdout(),
                    MoveTo(self.x + self.width - 1, self.y + i),
                    Print(chars.vertical),
                )?;
            }
        }

        execute!(
            stdout(),
            MoveTo(self.x, self.y + self.height - 1),
            Print(chars.bottom_left),
            Print(chars.horizontal.repeat(usize::from(self.width - 2))),
            Print(chars.bottom_right),
        )?;

        if self.colors.is_some() {
            execute!(
                stdout(),
                SetColors(Colors {
                    foreground: Some(Color::Reset),
                    background: Some(Color::Reset)
                })
            )?;
        }

        Ok(())
    }
}
