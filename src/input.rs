use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game::{World, GravityMode};

pub fn process_input(event_pump: &mut sdl2::EventPump, world: &mut World) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return false;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                world.bounciness = (world.bounciness + 0.05).min(1.2);
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                world.bounciness = (world.bounciness - 0.05).max(0.1);
            },
            Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                world.gravity_mode = match world.gravity_mode {
                    GravityMode::Centripetal => GravityMode::Vertical,
                    GravityMode::Vertical => GravityMode::Centripetal,
                };
            },
            _ => {}
        }
    }
    true
}
