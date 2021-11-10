/// This entire file is stolen from the tui-rs examples, as man, I see no reason
/// to write this badly for myself.  see also:
/// https://raw.githubusercontent.com/fdehau/tui-rs/master/examples/util/event.rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug)]
pub enum Event<I> {
    Input(I),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let _input_handle = {
            thread::spawn(move || {
                let mut stdin = termion::async_stdin().keys();
                loop {
                    if let Some(evt) = stdin.next() { match evt {
                        Ok(evt) => {
                            if let Err(err) = tx.send(Event::Input(evt)) {
                                eprintln!("{}", err);
                                return;
                            }
                        }
                        Err(e) => {
                            panic!("Event readeer errored: {:?}", e)
                        }
                    } }
                    thread::sleep(Duration::from_millis(100));
                }
            })
        };
        Events { rx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
