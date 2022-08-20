extern crate unicode_segmentation;

use std::io::stdout;

use crossterm::{cursor::MoveTo, execute, Result, style::Print};
use unicode_segmentation::UnicodeSegmentation;

use crate::traits::Draw;

/// A basic text component, allowing a string to be printed at a given location with an optional max length.
#[derive(Default)]
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

impl Draw for Text<'_> {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), MoveTo(self.x, self.y), Print(self.content))?;

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
