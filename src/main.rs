use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let window = video_subsystem.window("File Manager", 1600, 900).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode : Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
