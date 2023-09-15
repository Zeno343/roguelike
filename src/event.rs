pub use crossterm::event::{Event, KeyCode};
use {
    crossterm::event::{poll, read},
    std::time::Duration,
};

pub struct Events;

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if poll(Duration::from_millis(30)).expect("event poll failed") {
            Some(read().expect("event read failed"))
        } else {
            None
        }
    }
}
