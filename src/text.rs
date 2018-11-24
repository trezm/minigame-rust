use sdl2;
use sdl2::rect::Rect;

use rusttype;
use rusttype::{PositionedGlyph, FontCollection};

pub fn line(mut canvas: sdl2::render::Canvas<sdl2::video::Window>, text: &str, x: i32, y: i32) -> sdl2::render::Canvas<sdl2::video::Window> {
    let font_data = include_bytes!("../fonts/DIN Condensed Bold.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]).unwrap_or_else(|e| {
        panic!("error constructing a FontCollection from bytes: {}", e);
    });
    let font = collection.into_font() // only succeeds if collection consists of one font
        .unwrap_or_else(|e| {
            panic!("error turning FontCollection into a Font: {}", e);
        });

    let bpp = 4; // bytes per pixel
    let height: f32 = 72.0;
    let scale = rusttype::Scale { x: height, y: height };
    let start = rusttype::point(0.0, font.v_metrics(scale).ascent);
    let glyphs: Vec<PositionedGlyph> = font.layout(&text, scale, start).collect();
    let width = glyphs.iter()
        .rev()
        .filter_map(|glyph| {
            glyph.pixel_bounding_box()
                .map(|b_box| {
                    b_box.min.x as f32
                        + glyph.unpositioned()
                        .h_metrics()
                        .advance_width
                })
        }).next()
        .unwrap_or(height * 2.0).ceil() as usize;

    // NOTE: this is a little weird to have to cap the integer height of
    //       the texture to fit the (possibly) non-integer glyph height...
    //       but ohh well... *shrug*
    let height = height.ceil() as usize;

    let mut texture = canvas.texture_creator().create_texture(
        Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
        sdl2::render::TextureAccess::Streaming,
        width as u32,
        height as u32).unwrap();
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for glyph in glyphs {
            if let Some(b_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    // `v` is the pixel coverage of the glyph (aka alpha)
                    let alpha = (v * 255.0) as u8;
                    let x = x as i32 + b_box.min.x;
                    let y = y as i32 + b_box.min.y;

                    // the glyph coord could still be out of the texture
                    // bounds so we need to check it
                    if x >= 0 && x < width as i32
                        && y >= 0 && y < height as i32 {
                        let y_off: usize = y as usize * pitch;
                        let x_off: usize = x as usize * bpp;
                        let offset = y_off + x_off;
                        buffer[offset + 0] = alpha;
                        buffer[offset + 1] = 255_u8;
                        buffer[offset + 2] = 0;
                        buffer[offset + 3] = 255_u8;
                    }
                })
            }
        }
    }).unwrap();

    let width = width as u32;
    let height = height as u32;
    let src_rect = Rect::new(0, 0, width, height);
    let dst_rect = Rect::new(x, y, width, height);

    let _ = canvas.copy(&texture, src_rect, dst_rect);
    canvas
}
