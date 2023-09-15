mod entity;
mod event;
mod map;
mod screen;

use {
    entity::Entity,
    event::{Event, Events, KeyCode},
    screen::Screen,
    map::Map
};

fn main() {
    let mut screen = Screen::new().expect("terminal setup failed");

    let [w, h] = screen.size();

    let map = Map::new([w, h]);

    let center = [w / 2, h / 2];
    let player = Entity::new(center, '@');
    let entities = &[player];

    'main: loop {
        screen.clear().expect("screen clear failed");

        map.draw(&mut screen).expect("map draw failed");

        for entity in entities {
            entity.draw(&mut screen).expect("entity draw failed");
        }

        screen.present().expect("screen present failed");

        let events = Events;
        for event in events {
            if event == Event::Key(KeyCode::Esc.into()) {
                break 'main;
            }
        }
    }
}
