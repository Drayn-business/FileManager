mod filesystem;
mod render;

use std::{time::Duration, path::{Path, PathBuf}};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, mouse::{MouseButton, MouseWheelDirection}};

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

    let mut current_path: PathBuf = PathBuf::new().join("C:/");
    let mut current_filenames: Vec<String>;
    let mut current_textfields: Vec<Textfield> = vec![];
    let mut display_range: std::ops::Range<u32>;
    let mut display_offset = 0;
    let mut current_filenames_length: u32;

    let sdl2_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let video_subsystem = sdl2_context.video().unwrap();

    let mut font = ttf_context
        .load_font(Path::new(font_path), 20)
        .unwrap();

    let window = video_subsystem.window("File Manager", window_width, window_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'running: loop {
        let files_result = filesystem::get(current_path.clone());
        
        if files_result.is_none() {
            current_path = current_path.parent().unwrap().to_path_buf();
            current_filenames = filesystem::get(current_path.clone()).unwrap();
        }
        else {
            current_filenames = files_result.unwrap();
        }

        current_filenames_length = current_filenames.len() as u32;

        let (_, text_height) = font.size_of(current_filenames.first().unwrap().as_str()).unwrap();
        let display_range_end: u32;

        if current_filenames.len() <= (window_height / text_height) as usize {
            display_range_end = current_filenames.len() as u32;
        } else {
            display_range_end = (window_height / text_height) + display_offset;
        };

        display_range = (0 + display_offset)..display_range_end;
        
        current_filenames = current_filenames.split_at(display_range.start as usize).1.to_vec();
        current_filenames = current_filenames.split_at((display_range.end - display_range.start) as usize).0.to_vec();
        
        let mut text_y = 0;
        for filename in current_filenames.clone() {
            let text_x = 0;
            let (width, height) = font.size_of(filename.as_str()).unwrap();
            
            let textfield = Textfield::new(filename, text_x, text_y, width, height);
            current_textfields.push(textfield);

            text_y += height as i32 + 5;
        }
        
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
                            display_offset = 0;

                            if textfield.text == ".."{
                                current_path = current_path.parent().unwrap_or(&current_path).to_path_buf();
                            }
                            else {
                                current_path = current_path.join(textfield.text);
                            }
                        }
                    }
                },
                Event::MouseWheel { direction: MouseWheelDirection::Normal , y, ..} => {
                    if y == 1 && display_range.start > 0{
                        display_offset -= 1;
                    }
                    else if y == -1 && display_range.end < current_filenames_length as u32{
                        display_offset += 1;
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        for textfield in current_textfields {
            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();

            let underlined: bool = (textfield.x..=(textfield.x + textfield.width as i32)).contains(&x) && (textfield.y..=(textfield.y + textfield.height as i32)).contains(&y);

            render::render_text(&mut canvas, &mut font, textfield.text.as_str(), Color::RGB(200, 200, 200), textfield.x, textfield.y, underlined); 
        }

        current_textfields = vec![];

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
