use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    ttf::{Font, FontStyle},
    video::Window,
};

pub fn render_text(
    canvas: &mut Canvas<Window>,
    font: &mut Font,
    text: &str,
    text_color: Color,
    x: i32,
    y: i32,
    underlined: bool,
) {
    let texture_creator = canvas.texture_creator();

    if underlined {
        font.set_style(FontStyle::UNDERLINE);
    }

    let surface = font.render(text).blended(text_color).unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();
    canvas
        .copy(&texture, None, Some(Rect::new(x, y, width, height)))
        .unwrap();

    font.set_style(FontStyle::NORMAL);
}
