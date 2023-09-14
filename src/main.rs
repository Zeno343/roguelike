use crossterm::{
    cursor::{Hide, Show, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute, queue, style,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    io::{stdout, Write},
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let (width, height) = size()?;
    let [x, y] = [width / 2, height / 2];

    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    'main: loop {
        execute!(stdout, Clear(ClearType::All))?;
        queue!(
            stdout,
            MoveTo(x, y),
            style::PrintStyledContent("@".white()),
        )?;
        stdout.flush()?;

        if poll(Duration::from_millis(1))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Esc.into()) {
                break 'main;
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
