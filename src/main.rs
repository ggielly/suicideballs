extern crate sdl2;
extern crate rand;

use std::time::{Duration, Instant};
use std::collections::VecDeque;

mod game;
mod rendering;
mod input;

use game::{initialize_world, update_world, Config};
use rendering::render;
use input::process_input;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let config = Config::default();

    let window = video_subsystem.window("Suicide Balls", config.screen_width, config.screen_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut world = initialize_world(config);
    let mut frame_times = VecDeque::with_capacity(60);

    let time_step = Duration::from_secs_f64(1.0 / 60.0);
    let mut accumulator = Duration::new(0, 0);
    let mut current_time = Instant::now();

    'running: loop {
        let new_time = Instant::now();
        let frame_time = new_time - current_time;
        current_time = new_time;
        accumulator += frame_time;

        if !process_input(&mut event_pump, &mut world) {
            break 'running;
        }

        while accumulator >= time_step {
            update_world(&mut world);
            accumulator -= time_step;
        }

        // Calculate FPS
        frame_times.push_back(frame_time);
        if frame_times.len() > 60 {
            frame_times.pop_front();
        }
        let total_duration: Duration = frame_times.iter().sum();
        if !total_duration.is_zero() {
            world.fps = (frame_times.len() as f64 / total_duration.as_secs_f64()) as u32;
        } else {
            world.fps = 0;
        }

        render(&mut canvas, &world)?;
    }

    Ok(())
}