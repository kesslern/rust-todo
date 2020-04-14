use super::components::Drawable;
use super::components::Square;
use super::screen::Screen;
use crossterm::{
  event::{read, Event, KeyCode},
  Result,
};
use std::env::var;
use std::process::Command;
use tempfile::NamedTempFile;

pub struct TodoApp {
  screen: Screen,
  state: State,
}

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
    let screen = Screen::new()?;
    let state = State::new();

    Ok(TodoApp { screen, state })
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
    assert!(ecode.success());
    let contents = std::fs::read_to_string(file.path())?;
    Ok(contents)
  }

  pub fn run(&mut self) -> Result<()> {
    let square = Square::new();
    loop {
      self.screen.clear()?;
      square.draw(&self.screen)?;
      Screen::draw(&mut self.state)?;

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
