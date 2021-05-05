use crate::ui::Input;
use crossterm::event::{self, poll, read};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Input,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            exit_key: Input::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}
pub struct Events {
    rx: mpsc::Receiver<Event<Input>>,
    _tx: mpsc::Sender<Event<Input>>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Self {
        Events::with_config(Config {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    pub fn with_config(config: Config) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if poll(config.tick_rate).unwrap() {
                if let event::Event::Key(key) = read().unwrap() {
                    let input = Input::from(key);

                    event_tx.send(Event::Input(input)).unwrap();
                }
            }
            event_tx.send(Event::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<Input>, mpsc::RecvError> {
        self.rx.recv()
    }
}
