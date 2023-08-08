mod filesystem;
mod render;

use std::{time::Duration, path::Path};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() {
    let window_width = 1600;
    let window_height = 900;
    let font_path = "C:/Sources/FileManager/fonts/Roboto-Medium.ttf";

    let mut current_filenames: Vec<String> = vec![];

    let sdl2_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let video_subsystem = sdl2_context.video().unwrap();

    let font = ttf_context
        .load_font(Path::new(font_path), 20)
        .unwrap();

    let window = video_subsystem.window("File Manager", window_width, window_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode : Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    current_filenames = vec![];
                }
                _ => {}
            }
        }

        if current_filenames.is_empty() {
            current_filenames = filesystem::get("./");
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut text_y = 0;
        for path in current_filenames.clone() {
            let (_, height) = render::render_text(&mut canvas, &font, path.as_str(), Color::RGB(200, 200, 200), 0, text_y); 

            text_y += height as i32 + 5;
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
