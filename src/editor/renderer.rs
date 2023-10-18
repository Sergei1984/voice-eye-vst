use pixels::{Pixels, SurfaceTexture};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

use super::WINDOW_DIMENSIONS;

const WIDTH: u32 = (WINDOW_DIMENSIONS.0 * 2) as u32;
const HEIGHT: u32 = (WINDOW_DIMENSIONS.1 * 2) as u32;

pub struct VoiceEyeRenderer {
    pixels: Pixels,
}

impl VoiceEyeRenderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(handle: W) -> Self {
        let pixels =
            Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, &handle)).unwrap();

        Self { pixels }
    }

    pub fn draw_frame(&mut self) {
        let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
        pixmap.fill(Color::from_rgba8(100, 100, 255, 255));

        let circle = PathBuilder::from_circle(600.0, 600.0, 200.0).unwrap();
        let s = Stroke::default();

        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(255, 10, 10, 255));

        pixmap.fill_path(
            &circle,
            &paint,
            tiny_skia::FillRule::Winding,
            Transform::identity(),
            None,
        );

        let frame = self.pixels.get_frame();
        frame.copy_from_slice(pixmap.data());

        let _ = self.pixels.render();
    }
}
