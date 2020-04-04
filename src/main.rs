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
    todos: Vec<String>,
}

enum TodoEvent {
    AddTodo(String),
    ScreenResize(),
}

impl TodoState {
    pub fn new() -> Result<TodoState> {
        let (width, height) = crossterm::terminal::size()?;

        return Ok(TodoState {
            screen_size: ScreenSize { width, height },
            todos: vec![],
        });
    }

    pub fn dispatch(&mut self, event: TodoEvent) -> Result<()> {
        match event {
            TodoEvent::AddTodo(content) => {
                self.todos.push(content);
            }
            TodoEvent::ScreenResize() => {
                let (width, height) = crossterm::terminal::size()?;
                self.screen_size = ScreenSize { width, height };
            }
        }

        Ok(())
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

    pub fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All),)?;
        self.draw_size()?;
        self.draw_box(5, 5, 2, 2)?;
        execute!(stdout(), MoveTo(0, 0))?;

        for (idx, todo) in self.todos.iter().enumerate() {
            execute!(stdout(), MoveTo(0, idx as u16))?;
            print!("{}", todo);
        }

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
            Event::Key(x) if x == KeyCode::Char('a').into() => {
                state.dispatch(TodoEvent::AddTodo(input_from_file()?))?;
            }
            Event::Resize(_, _) => {
                state.dispatch(TodoEvent::ScreenResize())?;
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
