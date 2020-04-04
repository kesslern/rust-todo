use crossterm::Result;
use todo::app::TodoApp;

mod todo {
    mod screen {
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
    }

    pub mod app {
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
            ScreenResize(),
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
                    TodoEvent::ScreenResize() => {
                        self.screen.handle_resize()?;
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
                loop {
                    self.screen.draw(&mut self.state)?;

                    match read()? {
                        Event::Key(x) if x == KeyCode::Esc.into() => break,
                        Event::Key(x) if x == KeyCode::Char('a').into() => {
                            self.dispatch(TodoEvent::AddTodo(TodoApp::input_from_file()?))?;
                        }
                        Event::Resize(_, _) => {
                            self.dispatch(TodoEvent::ScreenResize())?;
                        }
                        _ => (),
                    }
                }

                Ok(())
            }
        }
    }
}

fn run() -> Result<()> {
    let mut app = TodoApp::new()?;
    app.run()?;

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
