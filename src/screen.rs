use super::app::State;
use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  style::Print,
  terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
  },
  Result,
};
use std::io::{stdout, Write};

pub struct Screen {}

impl Screen {
  pub fn new() -> Result<Screen> {
    enable_raw_mode()?;

    execute!(
      stdout(),
      EnterAlternateScreen,
      Clear(ClearType::All),
      MoveTo(0, 0),
      Hide,
      EnableMouseCapture,
    )?;

    Ok(Screen {})
  }

  fn draw_size() -> Result<()> {
    let (width, height) = size()?;
    let draw_x = width - 6;
    execute!(
      stdout(),
      MoveTo(draw_x, 0),
      Print("X: "),
      Print(width),
      MoveTo(draw_x, 1),
      Print("Y: "),
      Print(height),
    )?;

    Ok(())
  }

  pub fn clear(&self) -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
  }

  pub fn draw_string(string: &str, x: u16, y: u16, max_width: usize) -> Result<()> {
    let string: &str = if string.len() > max_width {
      string.split_at(max_width).0
    } else {
      &string[..]
    };
    execute!(stdout(), MoveTo(x, y), Print(string))?;

    Ok(())
  }

  pub fn draw(state: &State) -> Result<()> {
    Self::draw_size()?;
    Self::draw_string(&"1234567", 6, 6, 5)?;
    execute!(stdout(), MoveTo(0, 0))?;

    for (idx, todo) in state.todos.iter().enumerate() {
      execute!(stdout(), MoveTo(0, idx as u16), Print(todo))?;
    }

    stdout().flush()?;
    Ok(())
  }
}

impl Drop for Screen {
  fn drop(&mut self) {
    execute!(
      stdout(),
      Clear(ClearType::All),
      LeaveAlternateScreen,
      DisableMouseCapture,
      Show
    )
    .unwrap();

    disable_raw_mode().unwrap();
  }
}
