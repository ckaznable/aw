mod app;
mod cli;
mod wall;

use app::{Action, App};
use clap::Parser;
use cli::Args;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::error::Error;
use wall::ColorWall;

struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    args: Args,
}

impl Tui {
    fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Tui { terminal, args })
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut app = App::new(self.args);

        loop {
            self.terminal.draw(ui)?;

            let recv = app.next();
            match recv {
                Ok(action) => match action {
                    Action::Quit => break,
                    Action::Render => (),
                },
                Err(_) => break,
            }
        }

        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::new(Args::parse())?;
    tui.run()
}

fn ui(frame: &mut Frame) {
    frame.render_widget(ColorWall, frame.size());
}
