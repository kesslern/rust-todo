extern crate unicode_segmentation;

use crossterm::{cursor::MoveTo, execute, style::Print, Result};
use std::io::{stdout, Write};
use unicode_segmentation::UnicodeSegmentation;

pub trait Drawable {
  fn draw(&self) -> Result<()>;
}

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

pub struct Text<'a> {
  content: &'a str,
  max_length: Option<u16>,
  x: u16,
  y: u16,
}

impl Text<'_> {
  pub fn new() -> Text<'static> {
    Text {
      content: "Hello, world!",
      max_length: None,
      x: 0,
      y: 0,
    }
  }
}

impl Drawable for Text<'_> {
  fn draw(&self) -> Result<()> {
    execute!(stdout(), MoveTo(self.x, self.y), Print(self.content),);

    let max = self.max_length.unwrap_or(u16::max_value());
    let mut done = 0;
    let mut iter = UnicodeSegmentation::graphemes(self.content, true);
    while done < max {
      match iter.next() {
        Some(c) => {
          execute!(stdout(), Print(c))?;
          done += 1;
        }
        None => {
          break;
        }
      }
    }

    Ok(())
  }
}
