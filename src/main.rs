mod entity;
mod event;
mod screen;

use {
    entity::Entity,
    event::{Event, Events, KeyCode},
    screen::Screen,
};

fn main() {
    let mut screen = Screen::new().expect("terminal setup failed");

    let [width, height] = screen.size();
    let player = Entity::new([width / 2, height / 2], '@');
    let entities = &[player];

    'main: loop {
        screen.draw(entities).expect("drawing to terminal failed");

        let events = Events;
        for event in events {
            if event == Event::Key(KeyCode::Esc.into()) {
                break 'main;
            }
        }
    }
}
