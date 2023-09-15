mod event;
mod screen;

use {
    event::{Event, Events, KeyCode},
    screen::Screen,
};

fn main() {
    let mut screen = Screen::new().expect("terminal setup failed");

    'main: loop {
        screen.draw().expect("drawing to terminal failed");

        let events = Events;
        for event in events {
            if event == Event::Key(KeyCode::Esc.into()) {
                break 'main;
            }
        }
    }
}
