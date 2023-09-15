use {crate::screen::Screen, crossterm::style::Stylize};

pub struct Entity {
    position: [u16; 2],
    glyph: char,
}

impl Entity {
    pub fn new(position: [u16; 2], glyph: char) -> Self {
        Self { position, glyph }
    }

    pub fn draw(&self, screen: &mut Screen) -> std::io::Result<()> {
        screen.put_char(self.position, self.glyph.white())
    }
}
