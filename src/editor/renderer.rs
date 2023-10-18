use pixels::{Pixels, SurfaceTexture};

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
        let frame = self.pixels.get_frame();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= 10 && x < 10 + 100 && y >= 20 && y < 20 + 100;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }

        let _ = self.pixels.render();
    }
}
