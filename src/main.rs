pub mod app;
pub mod components;
pub mod constants;
pub mod primitives;
pub mod traits;

mod screen;

use app::TodoApp;
use crossterm::Result;

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
