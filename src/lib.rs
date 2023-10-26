use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use editor::VoiceEyeEditor;
use vst::{
    api::Supported,
    buffer::AudioBuffer,
    editor::Editor,
    plugin::{CanDo, HostCallback, Info, Plugin},
};

mod editor;
mod music;

/// Top level wrapper that exposes a full `vst::Plugin` implementation.
struct VoiceEyeVst {
    editor: Option<VoiceEyeEditor>,
    file: File,
}

impl VoiceEyeVst {
    /// Initializes the VST plugin, along with an optional `HostCallback` handle.
    fn new_maybe_host(_maybe_host: Option<HostCallback>) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("/Users/sergiitokariev/Downloads/voice-eye-log.txt")
            .unwrap();

        Self {
            editor: Some(VoiceEyeEditor::new()),
            file,
        }
    }
}

/// `vst::plugin_main` requires a `Default` implementation.
impl Default for VoiceEyeVst {
    fn default() -> Self {
        Self::new_maybe_host(None)
    }
}

/// Main `vst` plugin implementation.
impl Plugin for VoiceEyeVst {
    fn new(host: HostCallback) -> Self {
        Self::new_maybe_host(Some(host))
    }

    fn get_info(&self) -> Info {
        /// Use a hash of a string describing this plugin to avoid unique ID conflicts.
        const UNIQUE_ID_SEED: &str = "VoiceEye visualizer VST2 Plugin";
        static UNIQUE_ID: once_cell::sync::Lazy<i32> = once_cell::sync::Lazy::new(|| {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut s = DefaultHasher::new();
            UNIQUE_ID_SEED.hash(&mut s);
            s.finish() as i32
        });

        Info {
            name: "voice-eye".to_string(),
            vendor: "serhiitokariev".to_string(),
            unique_id: *UNIQUE_ID,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            preset_chunks: true,
            ..Info::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let inputs = buffer.input_count();
        let (input, _) = buffer.split();

        for ch in 0..inputs {
            let b = input.get(ch);
            let s = b
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join(",");

            let _ = writeln!(&self.file, "{}", s);
        }

        let _ = writeln!(&self.file, "");
    }

    fn can_do(&self, _can_do: CanDo) -> Supported {
        Supported::Maybe
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        self.editor
            .take()
            .map(|editor| Box::new(editor) as Box<dyn Editor>)
    }
}

vst::plugin_main!(VoiceEyeVst);
