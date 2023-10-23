use std::{iter::once, sync::Arc};

use cosmic_text::{
    fontdb::Database, Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache, Weight,
};
use pixels::{Error, Pixels};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Transform};

const FONT: &[u8] = include_bytes!("../assets/fonts/iosevka-Iosevka-medium.ttf");

use crate::{HEIGHT, WIDTH};

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

    // draw_text(state, &mut pixmap, "Hello, Rust! 🦀", 100, 100);
    draw_text(state, &mut pixmap, "Hello, Rust!", 100, 100);

    let frame = pixels.get_frame();
    frame.copy_from_slice(pixmap.data());

    Ok(())
}

fn draw_text(state: &mut State, pixmap: &mut Pixmap, text: &str, origin_x: i32, origin_y: i32) {
    println!("Fonts in db {}", state.font_system.db().len());
    for f in state.font_system.db().faces() {
        println!(
            "ID={} PSN={} Family={} Style={:?} Weight={:?} Stretch={:?}",
            f.id,
            f.post_script_name,
            f.families.iter().next().unwrap().0,
            f.style,
            f.weight,
            f.stretch
        );
    }

    let attrs = create_compatible_attrs(&state.font_system);

    let metrics = Metrics::new(30.0, 40.0);
    let mut buffer = Buffer::new(&mut state.font_system, metrics);

    let mut buffer = buffer.borrow_with(&mut state.font_system);

    buffer.set_size(WIDTH as f32, HEIGHT as f32);

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

pub struct State {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

pub fn create_state() -> State {
    // let mut fs =
    //     FontSystem::new_with_fonts(once(cosmic_text::fontdb::Source::Binary(Arc::new(FONT))));

    // println!("{}", fs.locale());

    let db = Database::new();
    let mut fs = FontSystem::new_with_locale_and_db("en-UA".to_string(), db);

    fs.db_mut().load_font_data(FONT.into());

    State {
        font_system: fs,
        swash_cache: SwashCache::new(),
    }
}

fn create_compatible_attrs<'a>(font_system: &FontSystem) -> Attrs<'a> {
    let font = font_system.db().faces().next().unwrap();

    Attrs::new()
        .style(font.style)
        .weight(font.weight)
        .stretch(font.stretch)
}
