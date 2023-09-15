mod event;
mod screen;

use {
    event::{Event, Events, KeyCode},
    screen::Screen,
};

fn main() -> std::io::Result<()> {
    let mut screen = Screen::new()?;

    'main: loop {
        screen.draw()?;

        let events = Events;
        for event in events {
            if event == Event::Key(KeyCode::Esc.into()) {
                break 'main;
            }
        }
    }

    Ok(())
}
