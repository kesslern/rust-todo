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

fn draw() -> Result<()> {
    let (_, y) = crossterm::terminal::size()?;
    execute!(stdout(), Clear(ClearType::All),)?;
    draw_size()?;

    for i in 0..y {
        execute!(stdout(), MoveTo(0, i))?;
        print!("{:?}", i);
    }

    stdout().flush()?;

    loop {
        match read()? {
            Event::Key(x) if x == KeyCode::Esc.into() => break,
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
