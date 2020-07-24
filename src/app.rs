use crate::components::Text;
use crate::primitives::Square;
use crate::screen::Screen;
use crate::traits::Draw;
use crossterm::{
    cursor::Hide,
    event::{read, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
    Result,
};
use std::env::var;
use std::io::{stdout, Write};
use std::process::Command;
use tempfile::NamedTempFile;

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
        let square = Square {
            x: 5,
            y: 5,
            width: 10,
            height: 15,
            ..Default::default()
        };
        let text = Text::new();
        loop {
            execute!(stdout(), Clear(ClearType::All))?;
            square.draw()?;
            text.draw()?;
            Screen::draw(&self.state)?;

            match read()? {
                Event::Key(x) if x == KeyCode::Esc.into() => break,
                Event::Key(x) if x == KeyCode::Char('a').into() => {
                    self.dispatch(TodoEvent::AddTodo(TodoApp::input_from_file()?))?;
                }
                _ => (),
            }
        }

        Ok(())
    }
}
