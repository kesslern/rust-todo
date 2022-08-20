use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::Event,
    execute,
    Result,
    style::Print,
};
use crossterm::event::MouseEventKind;

use crate::primitives::Square;
use crate::traits::{Draw, HandleEvent};

#[derive(Default)]
pub struct CountSquare {
    pub square: Square,
    pub count: u16,
}

#[derive(Default)]
pub struct CountSquareBuilder {
    square: Square,
}

impl CountSquareBuilder {
    pub fn new() -> CountSquareBuilder {
        CountSquareBuilder {
            ..Default::default()
        }
    }

    pub fn with_square(mut self, square: Square) -> Self {
        self.square = square;
        self
    }

    pub fn build(self) -> CountSquare {
        CountSquare {
            square: self.square,
            ..Default::default()
        }
    }
}

impl Draw for CountSquare {
    fn draw(&self) -> Result<()> {
        self.square.draw()?;
        execute!(
            stdout(),
            MoveTo(self.square.x + 1, self.square.y + 1),
            Print(self.count)
        )?;
        Ok(())
    }
}

impl HandleEvent for CountSquare {
    fn dispatch(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Mouse(event) => match event.kind {
                MouseEventKind::Up(_) => self.count += 1,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
