use {
    crossterm::{
        cursor::{Hide, MoveTo, Show},
        execute, queue,
        style::{PrintStyledContent, Stylize},
        terminal::{
            disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    },
    std::io::{stdout, Stdout, Write},
};

pub struct Screen {
    width: u16,
    height: u16,
    stdout: Stdout,
}

impl Screen {
    pub fn new() -> std::io::Result<Self> {
        let mut stdout = stdout();
        let (width, height) = size()?;

        execute!(stdout, EnterAlternateScreen, Hide)?;
        enable_raw_mode()?;

        Ok(Self {
            width,
            height,
            stdout,
        })
    }

    pub fn draw(&mut self) -> std::io::Result<()> {
        let [x, y] = [self.width / 2, self.height / 2];

        execute!(self.stdout, Clear(ClearType::All))?;

        queue!(self.stdout, MoveTo(x, y), PrintStyledContent("@".white()),)?;

        self.stdout.flush()
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        disable_raw_mode()
            .and_then(|()| execute!(self.stdout, LeaveAlternateScreen, Show))
            .expect("error dropping screen");
    }
}
