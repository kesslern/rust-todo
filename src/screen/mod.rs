use super::app::State;
use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
  },
  Result,
};
use std::io::{stdout, Write};

pub struct Screen {
  width: u16,
  height: u16,
}

impl Screen {
  pub fn new() -> Result<Screen> {
    let (width, height) = size()?;
    let screen = Screen { width, height };
    enable_raw_mode()?;

    execute!(
      stdout(),
      EnterAlternateScreen,
      Clear(ClearType::All),
      MoveTo(0, 0),
      Hide,
      EnableMouseCapture,
    )?;

    return Ok(screen);
  }

  fn draw_size(&self) -> Result<()> {
    let Screen { width, height } = self;
    let draw_x = width - 6;
    execute!(stdout(), MoveTo(draw_x, 0))?;
    print!("X: {:?}", width);
    execute!(stdout(), MoveTo(draw_x, 1))?;
    print!("Y: {:?}", height);

    Ok(())
  }

  fn draw_box(&self, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
    execute!(stdout(), MoveTo(x, y))?;
    print!("\u{2554}");
    for _ in 1..width - 1 {
      print!("\u{2550}");
    }
    print!("\u{2557}");

    for i in 1..height - 1 {
      execute!(stdout(), MoveTo(x, y + i))?;
      print!("\u{2551}");
      execute!(stdout(), MoveTo(x + width - 1, y + i))?;
      print!("\u{2551}");
    }
    execute!(stdout(), MoveTo(x, y + height - 1))?;
    print!("\u{255A}");
    for _ in 1..width - 1 {
      print!("\u{2550}");
    }
    print!("\u{255D}");

    Ok(())
  }

  pub fn handle_resize(&mut self) -> Result<()> {
    let (width, height) = size()?;
    self.width = width;
    self.height = height;
    Ok(())
  }

  pub fn clear(&self) -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
  }

  pub fn draw(&self, state: &State) -> Result<()> {
    self.clear()?;
    self.draw_size()?;
    self.draw_box(5, 5, 2, 2)?;
    execute!(stdout(), MoveTo(0, 0))?;

    for (idx, todo) in state.todos.iter().enumerate() {
      execute!(stdout(), MoveTo(0, idx as u16))?;
      print!("{}", todo);
    }

    for i in 1..self.height {
      execute!(stdout(), MoveTo(0, i))?;
      print!("{:?}", i);
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
