use crossterm::Result;

/// A drawable component.
pub trait Drawable {
    fn draw(&self) -> Result<()>;
}
