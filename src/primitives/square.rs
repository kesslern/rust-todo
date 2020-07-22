use crate::constants::CHARS;
use crate::traits::Draw;

use crossterm::{cursor::MoveTo, execute, style::Print, Result};
use std::io::{stdout, Write};

/// A square with a position and size.
#[derive(Default)]
pub struct Square {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Draw for Square {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), MoveTo(self.x, self.y))?;
        print!("{}", CHARS.lines.single.top_left);
        for _ in 1..self.width - 1 {
            print!("{}", CHARS.lines.single.horizontal);
        }
        print!("{}", CHARS.lines.single.top_right);

        for i in 1..self.height - 1 {
            execute!(
                stdout(),
                MoveTo(self.x, self.y + i),
                Print(CHARS.lines.single.vertical),
                MoveTo(self.x + self.width - 1, self.y + i),
                Print(CHARS.lines.single.vertical),
            )?;
        }
        execute!(
            stdout(),
            MoveTo(self.x, self.y + self.height - 1),
            Print(CHARS.lines.single.bottom_left)
        )?;
        for _ in 1..self.width - 1 {
            print!("{}", CHARS.lines.single.horizontal);
        }
        print!("{}", CHARS.lines.single.bottom_right);

        Ok(())
    }
}