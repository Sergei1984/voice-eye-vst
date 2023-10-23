use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use pixels::{Error, Pixels};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Transform};

use crate::{State, HEIGHT, WIDTH};

pub fn draw(state: &mut State, pixels: &mut Pixels) -> Result<(), Error> {
    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    pixmap.fill(Color::from_rgba8(100, 100, 255, 255));

    let circle = PathBuilder::from_circle(100.0, 100.0, 200.0).unwrap();

    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(255, 10, 10, 255));

    // pixmap.fill_path(
    //     &circle,
    //     &paint,
    //     tiny_skia::FillRule::Winding,
    //     Transform::identity(),
    //     None,
    // );

    draw_text(state, &mut pixmap, "Hello, Rust! 🦀", 100, 100);

    let frame = pixels.get_frame();
    frame.copy_from_slice(pixmap.data());

    Ok(())
}

fn draw_text(state: &mut State, pixmap: &mut Pixmap, text: &str, origin_x: i32, origin_y: i32) {
    println!("Fonts in db {}", state.font_system.db().len());
    for f in state.font_system.db().faces() {
        println!("ID={} PSN={}", f.id, f.post_script_name);
    }

    let metrics = Metrics::new(30.0, 40.0);
    let mut buffer = Buffer::new(&mut state.font_system, metrics);

    let mut buffer = buffer.borrow_with(&mut state.font_system);

    buffer.set_size(WIDTH as f32, HEIGHT as f32);

    let attrs = Attrs::new();
    attrs.attrs.family(cosmic_text::Family::Name("Oswald Bold"));

    buffer.set_text(text, attrs, Shaping::Advanced);
    buffer.shape_until_scroll();

    let text_color = cosmic_text::Color::rgb(0x0, 0xFF, 0x0);

    let mut paint = Paint::default();

    buffer.draw(&mut state.swash_cache, text_color, |x, y, w, h, color| {
        paint.set_color(Color::from_rgba8(
            color.r(),
            color.g(),
            color.b(),
            color.a(),
        ));

        pixmap.fill_rect(
            Rect::from_xywh(
                (x + origin_x) as f32,
                (y + origin_y) as f32,
                w as f32,
                h as f32,
            )
            .unwrap(),
            &paint,
            Transform::identity(),
            None,
        );
    });
}
