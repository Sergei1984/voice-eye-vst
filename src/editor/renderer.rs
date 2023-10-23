use cosmic_text::fontdb::Database;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use pixels::{Pixels, SurfaceTexture};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Transform};

use super::WINDOW_DIMENSIONS;

const WIDTH: u32 = (WINDOW_DIMENSIONS.0 * 2) as u32;
const HEIGHT: u32 = (WINDOW_DIMENSIONS.1 * 2) as u32;
const FONT: &[u8] = include_bytes!("../../assets/fonts/iosevka-Iosevka-medium.ttf");

pub struct VoiceEyeRenderer {
    pixels: Pixels,
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl VoiceEyeRenderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(handle: W) -> Self {
        let pixels =
            Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, &handle)).unwrap();

        let db = Database::new();
        let mut fs = FontSystem::new_with_locale_and_db("en-UA".to_string(), db);

        fs.db_mut().load_font_data(FONT.into());

        Self {
            pixels,
            font_system: fs,
            swash_cache: SwashCache::new(),
        }
    }

    pub fn draw_frame(&mut self) {
        let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
        pixmap.fill(Color::from_rgba8(100, 100, 255, 255));

        let circle = PathBuilder::from_circle(600.0, 600.0, 200.0).unwrap();

        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(255, 10, 10, 255));

        pixmap.fill_path(
            &circle,
            &paint,
            tiny_skia::FillRule::Winding,
            Transform::identity(),
            None,
        );

        self.draw_text(&mut pixmap, "Hello, Rust!", 100, 100);

        let frame = self.pixels.get_frame();
        frame.copy_from_slice(pixmap.data());

        let _ = self.pixels.render();
    }

    fn draw_text(&mut self, pixmap: &mut Pixmap, text: &str, origin_x: i32, origin_y: i32) {
        let attrs = create_compatible_attrs(&self.font_system);

        let metrics = Metrics::new(30.0, 40.0);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut self.font_system);

        buffer.set_size(WIDTH as f32, HEIGHT as f32);

        buffer.set_text(text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll();

        let text_color = cosmic_text::Color::rgb(0x0, 0xFF, 0x0);

        let mut paint = Paint::default();

        buffer.draw(&mut self.swash_cache, text_color, |x, y, w, h, color| {
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
}

fn create_compatible_attrs<'a>(font_system: &FontSystem) -> Attrs<'a> {
    let font = font_system.db().faces().next().unwrap();

    Attrs::new()
        .style(font.style)
        .weight(font.weight)
        .stretch(font.stretch)
}
