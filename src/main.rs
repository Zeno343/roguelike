mod screen;
use {
    std::time::Duration,
    screen::Screen,
    crossterm::{
    event::{poll, read, Event, KeyCode},
    }
};

fn main() -> std::io::Result<()> {
    let mut screen = Screen::new()?;

    'main: loop {
        screen.draw()?;

        if poll(Duration::from_millis(1))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Esc.into()) {
                break 'main;
            }
        }
    }

    Ok(())
}
