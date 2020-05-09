use super::app::State;
use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  style::Print,
  terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
  },
  Result,
};
use std::io::{stdout, Write};

/// The screen on which everything is drawn.
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
