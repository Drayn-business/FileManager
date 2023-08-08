mod filesystem;

use std::{time::Duration, path::Path};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::TextureQuery};

fn main() {
    let window_width = 1600;
    let window_height = 900;
    let font_path = "C:/Sources/FileManager/fonts/Roboto-Medium.ttf";

    filesystem::get("./");

    let sdl2_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let video_subsystem = sdl2_context.video().unwrap();

    let font = ttf_context
        .load_font(Path::new(font_path), 30)
        .unwrap();

    let window = video_subsystem.window("File Manager", window_width, window_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'running: loop {
        let texture_creator = canvas.texture_creator();

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

        let surface = font
                .render("Reset game")
                .blended(Color::RGB(30, 30, 30))
                .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height))).unwrap();  

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
