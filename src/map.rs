use {crate::screen::Screen, crossterm::style::Stylize};

const FLOOR: char = '.';
const WALL: char = '\u{2588}';

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    blocks_movement: bool,
    blocks_sight: bool,
}

pub struct Map {
    tiles: Vec<Tile>,
    width: u16,
    height: u16,
}

impl Map {
    pub fn new([width, height]: [u16; 2]) -> Self {
        Self {
            tiles: vec![
                Tile {
                    blocks_movement: false,
                    blocks_sight: false
                };
                (width * height) as usize
            ],
            width,
            height,
        }
    }

    pub fn draw(&self, screen: &mut Screen) -> std::io::Result<()> {
        for x in 0..self.width {
            for y in 0..self.height {
                if self[[x, y]].blocks_sight {
                    screen.put_char([x, y], WALL.white())?;
                } else {
                    screen.put_char([x, y], FLOOR.grey())?;
                }
            }
        }

        Ok(())
    }
}

impl std::ops::Index<[u16; 2]> for Map {
    type Output = Tile;
    fn index(&self, [x, y]: [u16; 2]) -> &Self::Output {
        &self.tiles[(y * self.width + x) as usize]
    }
}
