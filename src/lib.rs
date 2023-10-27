use std::sync::Arc;

use editor::VoiceEyeEditor;
use futures::lock::Mutex;
use model::MeasureModel;
use music::MyPitchDetector;
use vst::{
    api::Supported,
    buffer::AudioBuffer,
    editor::Editor,
    plugin::{CanDo, HostCallback, Info, Plugin},
};

mod editor;
mod model;
mod music;

/// Top level wrapper that exposes a full `vst::Plugin` implementation.
struct VoiceEyeVst {
    editor: Option<VoiceEyeEditor>,
    pitch_detector: MyPitchDetector,
    sample_rate: f32,
}

impl VoiceEyeVst {
    /// Initializes the VST plugin, along with an optional `HostCallback` handle.
    fn new_maybe_host(_maybe_host: Option<HostCallback>) -> Self {
        let model = Arc::new(Mutex::new(MeasureModel::new()));

        Self {
            editor: Some(VoiceEyeEditor::new(Arc::clone(&model))),
            pitch_detector: MyPitchDetector::new(Arc::clone(&model)),
            sample_rate: 0.0,
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

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (input, _) = buffer.split();
        let samples = input.get(0);

        self.pitch_detector.detect(self.sample_rate, samples);
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
