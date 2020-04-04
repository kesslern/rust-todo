use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Result,
};
use std::env::var;
use std::process::Command;
use tempfile::NamedTempFile;

use std::io::{stdout, Write};

struct ScreenSize {
    width: u16,
    height: u16,
}

struct TodoState {
    screen_size: ScreenSize,
    content: Option<String>,
}

enum TodoEvent {
    ContentChanged(String),
    ResizeScreen(ScreenSize),
}

impl TodoState {
    pub fn new() -> Result<TodoState> {
        let (width, height) = crossterm::terminal::size()?;

        return Ok(TodoState {
            content: None,
            screen_size: ScreenSize { width, height },
        });
    }

    pub fn dispatch(&mut self, event: TodoEvent) {
        match event {
            TodoEvent::ContentChanged(content) => {
                println!("content changed");
                self.content = Some(content);
            }
            TodoEvent::ResizeScreen(screen_size) => {
                println!("resized");
                self.screen_size = screen_size;
            }
        }
    }

    fn draw_size(&self) -> Result<()> {
        let ScreenSize { width, height } = self.screen_size;
        let draw_x = width - 6;
        execute!(stdout(), MoveTo(draw_x, 0))?;
        print!("X: {:?}", width);
        execute!(stdout(), MoveTo(draw_x, 1))?;
        print!("Y: {:?}", height);

        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All),)?;
        self.draw_size()?;
        execute!(stdout(), MoveTo(0, 0))?;
        print!("{:?}", self.content);

        for i in 1..self.screen_size.height {
            execute!(stdout(), MoveTo(0, i))?;
            print!("{:?}", i);
        }

        stdout().flush()?;
        Ok(())
    }
}

fn init() -> Result<()> {
    enable_raw_mode()?;

    execute!(
        stdout(),
        EnterAlternateScreen,
        crossterm::terminal::Clear(ClearType::All),
        MoveTo(0, 0),
        Hide,
        EnableMouseCapture,
    )?;

    Ok(())
}

fn cleanup() -> Result<()> {
    execute!(
        stdout(),
        crossterm::terminal::Clear(ClearType::All),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;

    disable_raw_mode()?;

    Ok(())
}

fn input_from_file() -> Result<String> {
    let file = NamedTempFile::new()?;
    let mut child = Command::new(var("EDITOR").unwrap())
        .arg(file.path())
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");
    assert!(ecode.success());
    let contents = std::fs::read_to_string(file.path())?;
    Ok(contents)
}

fn run_loop() -> Result<()> {
    let mut state = TodoState::new()?;

    loop {
        state.draw()?;

        match read()? {
            Event::Key(x) if x == KeyCode::Esc.into() => break,
            Event::Key(x) if x == KeyCode::Char('t').into() => {
                state.dispatch(TodoEvent::ContentChanged(input_from_file()?));
            }
            Event::Resize(_, _) => {
                let (width, height) = crossterm::terminal::size()?;
                state.dispatch(TodoEvent::ResizeScreen(ScreenSize { width, height }));
            }
            _ => (),
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    init()?;
    run_loop()?;
    cleanup()?;

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            cleanup().unwrap();
            eprintln!("error: {:?}", err);
            1
        }
    });
}
