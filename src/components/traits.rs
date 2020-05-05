use crossterm::Result;

pub trait Drawable {
  fn draw(&self) -> Result<()>;
}
