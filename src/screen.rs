use {
    super::entity::Entity,
    crossterm::{
        cursor::{Hide, MoveTo, Show},
        execute, queue,
        style::{PrintStyledContent, StyledContent},
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

    pub fn draw(&mut self, entities: &[Entity]) -> std::io::Result<()> {
        execute!(self.stdout, Clear(ClearType::All))?;

        for entity in entities {
            entity.draw(self)?;
        }

        self.stdout.flush()
    }

    pub fn put_char(
        &mut self,
        [x, y]: [u16; 2],
        symbol: StyledContent<char>,
    ) -> std::io::Result<()> {
        queue!(self.stdout, MoveTo(x, y), PrintStyledContent(symbol))
    }

    pub fn size(&self) -> [u16; 2] {
        [self.width, self.height]
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        disable_raw_mode()
            .and_then(|()| execute!(self.stdout, LeaveAlternateScreen, Show))
            .expect("error dropping screen");
    }
}
