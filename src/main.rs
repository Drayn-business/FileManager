mod filesystem;
mod render;

use std::{time::Duration, path::{Path, PathBuf}, env};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, mouse::MouseButton};

#[derive(Clone)]
struct Textfield {
    text: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32
}

impl Textfield {
    fn new(text: String, x: i32, y: i32, width: u32, height: u32) -> Self {
        return Textfield { text, x, y, width, height };
    }
}

fn main() {
    let window_width = 1600;
    let window_height = 900;
    let font_path = "C:/Sources/FileManager/fonts/Roboto-Medium.ttf";

    let mut current_path: PathBuf = env::current_dir().unwrap();
    let mut current_filenames: Vec<String> = vec![];
    let mut current_textfields: Vec<Textfield> = vec![];

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
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    for textfield in current_textfields.clone() {
                        if (textfield.x..=(textfield.x + textfield.width as i32)).contains(&x) && 
                           (textfield.y..=(textfield.y + textfield.height as i32)).contains(&y) {
                            if textfield.text == ".."{
                                current_path = current_path.parent().unwrap_or(&current_path).to_path_buf();
                            }
                            else {
                                current_path = current_path.join(textfield.text);
                            }

                            current_filenames = vec![];
                            current_textfields = vec![];
                        }
                    }
                }
                _ => {}
            }
        }

        if current_filenames.is_empty() {
            let files_result = filesystem::get(current_path.clone());
            
            if files_result.is_none() {
                current_path = current_path.parent().unwrap().to_path_buf();
                current_filenames = filesystem::get(current_path.clone()).unwrap();
            }
            else {
                current_filenames = files_result.unwrap();
            }
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        current_textfields = vec![];
        let mut text_y = 0;
        for filename in current_filenames.clone() {
            let text_x = 0;
            let (width, height) = render::render_text(&mut canvas, &font, filename.as_str(), Color::RGB(200, 200, 200), text_x, text_y); 

            let textfield = Textfield::new(filename, text_x, text_y, width, height);
            current_textfields.push(textfield);

            text_y += height as i32 + 5;
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
