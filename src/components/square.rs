use super::Drawable;
use crossterm::{cursor::MoveTo, execute, style::Print, Result};
use std::io::{stdout, Write};

/// A square with a position and size.
#[derive(Default)]
pub struct Square {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Square {
    pub fn new() -> Square {
        Square {
            x: 0,
            y: 0,
            width: 5,
            height: 5,
        }
    }
}

impl Drawable for Square {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), MoveTo(self.x, self.y))?;
        print!("\u{2554}");
        for _ in 1..self.width - 1 {
            print!("\u{2550}");
        }
        print!("\u{2557}");

        for i in 1..self.height - 1 {
            execute!(
                stdout(),
                MoveTo(self.x, self.y + i),
                Print("\u{2551}"),
                MoveTo(self.x + self.width - 1, self.y + i),
                Print("\u{2551}"),
            )?;
        }
        execute!(
            stdout(),
            MoveTo(self.x, self.y + self.height - 1),
            Print("\u{255A}")
        )?;
        for _ in 1..self.width - 1 {
            print!("\u{2550}");
        }
        print!("\u{255D}");

        Ok(())
    }
}
