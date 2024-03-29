use std::env::var;
use std::io::stdout;
use std::process::Command;

use crossterm::{
    cursor::Hide,
    event::{EnableMouseCapture, Event, KeyCode, read},
    execute,
    Result,
    style::{Color, Colors},
    terminal::{Clear, ClearType},
};
use tempfile::NamedTempFile;

use crate::components::{CountSquareBuilder, Text};
use crate::primitives::SquareBuilder;
use crate::screen::Screen;
use crate::traits::{Draw, HandleEvent};

pub struct TodoApp {
    _screen: Screen,
    state: State,
}

#[derive(Default)]
pub struct State {
    pub todos: Vec<String>,
}

impl State {
    pub fn new() -> State {
        State { todos: vec![] }
    }
}

pub enum TodoEvent {
    AddTodo(String),
}

impl TodoApp {
    pub fn new() -> Result<TodoApp> {
        let _screen = Screen::new()?;
        let state = State::new();

        Ok(TodoApp { _screen, state })
    }

    pub fn dispatch(&mut self, event: TodoEvent) -> Result<()> {
        match event {
            TodoEvent::AddTodo(content) => {
                self.state.todos.push(content);
            }
        }

        Ok(())
    }

    fn input_from_file() -> Result<String> {
        let file = NamedTempFile::new()?;
        let mut child = Command::new(var("EDITOR").unwrap())
            .arg(file.path())
            .spawn()
            .expect("failed to execute child");

        let ecode = child.wait().expect("failed to wait on child");
        execute!(stdout(), Hide, EnableMouseCapture,)?;
        assert!(ecode.success());
        let contents = std::fs::read_to_string(file.path())?;
        Ok(contents)
    }

    pub fn run(&mut self) -> Result<()> {
        let square = SquareBuilder::new(5, 5, 10, 15)
            .is_filled(true)
            .with_colors(Colors {
                foreground: Some(Color::Rgb { r: 0, g: 0, b: 255 }),
                background: Some(Color::Rgb {
                    r: 200,
                    g: 0,
                    b: 200,
                }),
            })
            .build();

        let mut count_square = CountSquareBuilder::new()
            .with_square(
                SquareBuilder::new(25, 5, 10, 15)
                    .is_filled(true)
                    .with_colors(Colors {
                        foreground: Some(Color::Rgb { r: 0, g: 0, b: 255 }),
                        background: Some(Color::Rgb {
                            r: 200,
                            g: 0,
                            b: 200,
                        }),
                    })
                    .build(),
            )
            .build();

        let text = Text::new();
        loop {
            execute!(stdout(), Clear(ClearType::All))?;
            square.draw()?;
            text.draw()?;
            Screen::draw(&self.state)?;
            count_square.draw()?;

            let event = read()?;

            match event {
                Event::Key(x) if x == KeyCode::Esc.into() => break,
                Event::Key(x) if x == KeyCode::Char('a').into() => {
                    self.dispatch(TodoEvent::AddTodo(TodoApp::input_from_file()?))?;
                }
                _ => (),
            }

            count_square.dispatch(event)?;
        }

        Ok(())
    }
}
