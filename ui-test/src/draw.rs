use pixels::{Error, Pixels};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Transform};

use crate::{HEIGHT, WIDTH};

pub fn draw(pixels: &mut Pixels) -> Result<(), Error> {
    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    pixmap.fill(Color::from_rgba8(100, 100, 255, 255));

    let circle = PathBuilder::from_circle(100.0, 100.0, 200.0).unwrap();

    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(255, 10, 10, 255));

    pixmap.fill_path(
        &circle,
        &paint,
        tiny_skia::FillRule::Winding,
        Transform::identity(),
        None,
    );

    // self.draw_text(&mut pixmap, "Hello, Rust! 🦀", 100, 100);

    let frame = pixels.get_frame();
    frame.copy_from_slice(pixmap.data());

    Ok(())
}
