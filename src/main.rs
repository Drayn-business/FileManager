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
    const WINDOW_WIDTH: u32 = 1600;
    const WINDOW_HEIGHT: u32 = 900;
    const FONT_PATH: &str = "C:/Sources/FileManager/fonts/Roboto-Medium.ttf";
    const BACKGROUND_COLOR: Color = Color::RGB(30, 30, 30);
    const TEXT_COLOR: Color = Color::RGB(200, 200, 200);

    let mut current_path: PathBuf = PathBuf::new().join("C:/");
    let mut filenames: Vec<String> = vec![];
    let mut display_offset = 0;

    let sdl2_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let video_subsystem = sdl2_context.video().unwrap();

    let mut font = ttf_context
        .load_font(Path::new(FONT_PATH), 20)
        .unwrap();

    let window = video_subsystem.window("File Manager", WINDOW_WIDTH, WINDOW_HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'running: loop {
        let mut textfields: Vec<Textfield> = vec![];

        if filenames.is_empty(){
            let mut files_result = filesystem::get(current_path.clone());
            
            if files_result.is_none() {
                current_path = current_path.parent().unwrap().to_path_buf();
                files_result = filesystem::get(current_path.clone());
            }

            filenames = files_result.unwrap();
        }

        let (_, text_height) = font.size_of(filenames.first().unwrap().as_str()).unwrap();
        let row_amount = WINDOW_HEIGHT / text_height;
        
        let display_range_end: u32;

        if filenames.len() <= row_amount as usize {
            display_range_end = filenames.len() as u32;
        } else {
            display_range_end = row_amount + display_offset;
        };

        let display_range = (0 + display_offset)..display_range_end;
        
        let mut text_y = 0;
        for i in display_range.clone() {
            let filename = &filenames[i as usize];
            
            let text_x = 0;
            let (width, height) = font.size_of(filename).unwrap();
            
            let textfield = Textfield::new(filename.to_string(), text_x, text_y, width, height);
            textfields.push(textfield);

            text_y += height as i32 + 5;
        }
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode : Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    for textfield in textfields.clone() {
                        let textfield_vertical_range = textfield.x..=(textfield.x + textfield.width as i32);
                        let textfield_horizontal_range = textfield.y..=(textfield.y + textfield.height as i32);

                        if textfield_vertical_range.contains(&x) && textfield_horizontal_range.contains(&y) {
                            filenames = vec![];
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
                    else if y == -1 && display_range.end < filenames.len() as u32{
                        display_offset += 1;
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        for textfield in textfields {
            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();

            let textfield_vertical_range = textfield.x..=(textfield.x + textfield.width as i32);
            let textfield_horizontal_range = textfield.y..=(textfield.y + textfield.height as i32);

            let underlined: bool = textfield_vertical_range.contains(&x) && textfield_horizontal_range.contains(&y);

            render::render_text(&mut canvas, &mut font, textfield.text.as_str(), TEXT_COLOR, textfield.x, textfield.y, underlined); 
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
