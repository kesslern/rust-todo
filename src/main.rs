use crossterm::Result;
use rust_todo::app::TodoApp;

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
