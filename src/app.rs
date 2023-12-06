use crossbeam_channel::{Sender, Receiver};
use crossterm::event::{self, Event, KeyCode};
pub enum Action {
    Quit,
    Render,
}

pub struct App {
    action_tx: Sender<Action>,
    action_rx: Receiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = crossbeam_channel::bounded(1);

        let app = Self { action_tx, action_rx };
        app.handle_terminal_event();

        app
    }

    fn handle_terminal_event(&self) {
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }

                    let send_res = match key.code {
                        KeyCode::Char('q') => tx.send(Action::Quit),
                        _ => tx.send(Action::Render),
                    };

                    if send_res.is_err() {
                        break
                    }
                }
            };
        });
    }

    pub fn next(&mut self) -> Result<Action, crossbeam_channel::RecvError> {
        self.action_rx.recv()
    }
}