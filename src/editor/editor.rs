use core::ffi::c_void;
use futures::lock::Mutex;
use std::sync::Arc;
use vst::editor::Editor;
use vst_window::{setup, EventSource};

use crate::model::MeasureModel;

use super::renderer::VoiceEyeRenderer;

pub const WINDOW_DIMENSIONS: (i32, i32) = (1200, 600);

pub struct VoiceEyeEditor {
    renderer: Option<VoiceEyeRenderer>,
    window_events: Option<EventSource>,
    model: Arc<Mutex<MeasureModel>>,
}

impl VoiceEyeEditor {
    pub fn new(model: Arc<Mutex<MeasureModel>>) -> Self {
        Self {
            renderer: None,
            window_events: None,
            model,
        }
    }
}

impl Editor for VoiceEyeEditor {
    fn size(&self) -> (i32, i32) {
        (WINDOW_DIMENSIONS.0 as i32, WINDOW_DIMENSIONS.1 as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        if self.window_events.is_none() {
            let (window_handle, event_source) = setup(parent, WINDOW_DIMENSIONS);
            self.renderer = Some(VoiceEyeRenderer::new(
                window_handle,
                Arc::clone(&self.model),
            ));
            self.window_events = Some(event_source);
            true
        } else {
            false
        }
    }

    fn is_open(&mut self) -> bool {
        self.window_events.is_some()
    }

    fn close(&mut self) {
        drop(self.renderer.take());
        drop(self.window_events.take());
    }

    fn idle(&mut self) {
        if let Some(window_events) = &mut self.window_events {
            while let Some(event) = window_events.poll_event() {
                match event {
                    //     WindowEvent::MouseClick(_) => println!("Click!"),
                    _ => (),
                }
            }
        }
        if let Some(renderer) = &mut self.renderer {
            renderer.draw_frame();
        }
    }
}
