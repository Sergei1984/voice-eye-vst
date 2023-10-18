pub struct VoiceEyeRenderer;

impl VoiceEyeRenderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(_handle: W) -> Self {
        Self
    }
    pub fn draw_frame(&mut self) {
        /* ... */
    }
}
