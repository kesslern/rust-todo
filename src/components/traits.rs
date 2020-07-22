use crossterm::Result;

/// A drawable component.
pub trait Draw {
    fn draw(&self) -> Result<()>;
}
