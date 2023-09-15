mod entity;
mod event;
mod map;
mod screen;

use {
    crossterm::event::{read, Event, KeyCode},
    entity::Entity,
    map::Map,
    screen::Screen,
    std::time::Duration,
};

fn main() {
    let mut screen = Screen::new().expect("terminal setup failed");

    let [w, h] = screen.size();

    let map = Map::new([w, h]);

    let center = [w / 2, h / 2];
    let player = Entity::new(center, '@');
    let entities = &[player];

    draw(&mut screen, entities, &map);

    'main: loop {
        let event = read().expect("event read failed");

        if event == Event::Key(KeyCode::Esc.into()) {
            break 'main;
        }

        draw(&mut screen, entities, &map);
    }
}

fn draw(screen: &mut Screen, entities: &[Entity], map: &Map) {
    screen.clear().expect("screen clear failed");

    map.draw(screen).expect("map draw failed");

    for entity in entities {
        entity.draw(screen).expect("entity draw failed");
    }

    screen.present().expect("screen present failed");
}
