use std::{
    fs::{File, OpenOptions},
    os::{fd::{OwnedFd, AsRawFd, RawFd, AsFd, BorrowedFd}, unix::fs::OpenOptionsExt},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crossbeam_channel::{Receiver, Sender};
use crossterm::event::{self, Event, KeyCode};
use input::{Libinput, LibinputInterface, event::KeyboardEvent};
use nix::poll::{PollFd, PollFlags, poll};

use crate::cli::Args;

pub enum Action {
    Quit,
    Render,
}

pub struct App {
    action_tx: Sender<Action>,
    action_rx: Receiver<Action>,
    should_quit: Arc<AtomicBool>,
    is_focused: Arc<AtomicBool>,
    args: Args,
}

impl App {
    pub fn new(args: Args) -> Self {
        let (action_tx, action_rx) = crossbeam_channel::bounded(1);
        let should_quit = Arc::new(AtomicBool::new(false));
        let is_focused = Arc::new(AtomicBool::new(true));

        let app = Self {
            action_tx,
            action_rx,
            args,
            should_quit,
            is_focused,
        };
        app.handle_terminal_event();

        if args.global {
            app.handle_global_event();
        }

        app
    }

    fn handle_terminal_event(&self) {
        let tx = self.action_tx.clone();
        let should_quit = self.should_quit.clone();
        let is_focused = self.is_focused.clone();
        let args = self.args;

        tokio::spawn(async move {
            while !should_quit.load(Ordering::SeqCst) {
                let Ok(_event) = event::read() else {
                    break;
                };

                match _event {
                    Event::FocusGained => {
                        is_focused.store(true, Ordering::SeqCst);
                    }
                    Event::FocusLost => {
                        is_focused.store(false, Ordering::SeqCst);
                    }
                    Event::Resize(_, _) => {
                        let Ok(_) = tx.send(Action::Render) else {
                            break;
                        };
                    }
                    Event::Key(key) => {
                        if key.kind == event::KeyEventKind::Release {
                            continue;
                        }

                        if !is_focused.load(Ordering::SeqCst) {
                            continue;
                        }

                        let res = match key.code {
                            KeyCode::Char('q') => {
                                should_quit.store(true, Ordering::SeqCst);
                                tx.send(Action::Quit)
                            }
                            _ => {
                                if !args.global {
                                    tx.send(Action::Render)
                                } else {
                                    Ok(())
                                }
                            }
                        };

                        if res.is_err() || should_quit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                    _ => (),
                };
            }
        });
    }

    fn handle_global_event(&self) {
        let tx = self.action_tx.clone();
        let should_quit = self.should_quit.clone();
        let is_focused = self.is_focused.clone();
        let args = self.args;

        tokio::spawn(async move {
            let mut input = Libinput::new_with_udev(UnixInputInterface);
            input.udev_assign_seat("seat0").unwrap();
            let mut is_release = false;
            let raw_fd = CustomFD(input.as_raw_fd());
            let pollfd = PollFd::new(&raw_fd, PollFlags::POLLIN);

            'root: while poll(&mut [pollfd], -1).is_ok() && !should_quit.load(Ordering::SeqCst) {
                let Ok(_) = input.dispatch() else { break };

                if args.only_focused && !is_focused.load(Ordering::SeqCst) {
                    continue;
                }

                for event in &mut input {
                    if let input::Event::Keyboard(KeyboardEvent::Key(_)) = event {
                        if is_release {
                            is_release = false;
                            continue 'root;
                        }

                        let _ = tx.send(Action::Render);
                        is_release = true;
                        continue 'root;
                    }
                }
            }
        });
    }

    pub fn next(&mut self) -> Result<Action, crossbeam_channel::RecvError> {
        self.action_rx.recv()
    }
}

struct UnixInputInterface;

impl LibinputInterface for UnixInputInterface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read(true)
            .write(false)
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

struct CustomFD(RawFd);
impl AsFd for CustomFD {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.0) }
    }
}