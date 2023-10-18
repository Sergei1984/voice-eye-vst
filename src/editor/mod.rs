use vst::editor::Editor;
use vst_window::{setup, EditorWindow};

use self::graphics::Renderer;

mod graphics;

const SCALE: f64 = 0.5;

/// Actual pixel width of the editor window.
pub(super) const SIZE_X: usize = (2400 as f64 * SCALE) as usize;
/// Actual pixel height of the editor window.
pub(super) const SIZE_Y: usize = (1200 as f64 * SCALE) as usize;

pub struct PluginEditor {
    renderer: Option<graphics::Renderer>,
    is_open: bool,
}

impl PluginEditor {
    pub fn new() -> PluginEditor {
        Self {
            is_open: false,
            renderer: None,
        }
    }
}

impl Editor for PluginEditor {
    fn size(&self) -> (i32, i32) {
        (SIZE_X as i32, SIZE_Y as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut core::ffi::c_void) -> bool {
        if self.is_open {
            self.is_open = true;

            let (window, _) = setup(parent, (SIZE_X as i32, SIZE_Y as i32));
            self.renderer = Some(Renderer::new(window));

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

    fn idle(&mut self) {
        if self.is_open {
            if let Some(r) = &mut self.renderer {
                r.draw_frame();
            }
        }
    }
}
