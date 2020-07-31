use crate::primitives::Square;
use crate::traits::{Draw, HandleEvent};
use crossterm::{
    cursor::MoveTo,
    event::{Event, MouseEvent},
    execute,
    style::Print,
    Result,
};
use std::io::{stdout, Write};

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
    fn dispatch(&mut self, event: crossterm::event::Event) -> Result<()> {
        match event {
            Event::Mouse(event) => match event {
                MouseEvent::Up(_, _, _, _) => self.count += 1,
                MouseEvent::Down(_, _, _, _) => {}
                MouseEvent::Drag(_, _, _, _) => {}
                MouseEvent::ScrollDown(_, _, _) => {}
                MouseEvent::ScrollUp(_, _, _) => {}
            },
            Event::Key(_) => {}
            Event::Resize(_, _) => {}
        }
        Ok(())
    }
}
