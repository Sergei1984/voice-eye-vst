use vst::editor::Editor;

const SCALE: f64 = 0.5;

/// Actual pixel width of the editor window.
pub(super) const SIZE_X: usize = (2400 as f64 * SCALE) as usize;
/// Actual pixel height of the editor window.
pub(super) const SIZE_Y: usize = (1200 as f64 * SCALE) as usize;

pub struct PluginEditor {
    is_open: bool,
}

impl PluginEditor {
    pub fn new() -> PluginEditor {
        Self { is_open: false }
    }
}

impl Editor for PluginEditor {
    fn size(&self) -> (i32, i32) {
        (SIZE_X as i32, SIZE_Y as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, _parent: *mut core::ffi::c_void) -> bool {
        if self.is_open {
            self.is_open = true;
            true
        } else {
            false
        }
    }

    fn close(&mut self) {
        if self.is_open {
            self.is_open = false;
        }
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn idle(&mut self) {}
}
