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

fn draw_size() -> Result<()> {
    let (x, y) = crossterm::terminal::size()?;
    let draw_x = x - 6;
    execute!(stdout(), MoveTo(draw_x, 0))?;
    print!("X: {:?}", x);
    execute!(stdout(), MoveTo(draw_x, 1))?;
    print!("Y: {:?}", y);

    Ok(())
}

fn test() -> Result<String> {
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

fn draw() -> Result<()> {
    let mut contents: String = "".to_string();

    loop {
        let (_, y) = crossterm::terminal::size()?;
        execute!(stdout(), Clear(ClearType::All),)?;
        draw_size()?;

        execute!(stdout(), MoveTo(0, 0))?;
        print!("{:?}", contents);

        for i in 1..y {
            execute!(stdout(), MoveTo(0, i))?;
            print!("{:?}", i);
        }

        stdout().flush()?;
        match read()? {
            Event::Key(x) if x == KeyCode::Esc.into() => break,
            Event::Key(x) if x == KeyCode::Char('t').into() => {
                contents = test()?;
            }
            Event::Resize(_, _) => draw()?,
            _ => (),
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    init()?;
    draw()?;
    cleanup()?;

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
