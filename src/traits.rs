use crossterm::{event::Event, Result};

/// A drawable component.
pub trait Draw {
    fn draw(&self) -> Result<()>;
}

pub trait HandleEvent {
    fn dispatch(&mut self, event: Event) -> Result<()>;
}
