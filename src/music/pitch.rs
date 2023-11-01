use futures::lock::Mutex;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use std::{
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::model::MeasureModel;

pub struct MyPitchDetector {
    detector: McLeodDetector<f32>,
    model: Arc<Mutex<MeasureModel>>,
}

impl MyPitchDetector {
    pub fn new(model: Arc<Mutex<MeasureModel>>) -> Self {
        MyPitchDetector {
            detector: McLeodDetector::new(512, 256),
            model: model,
        }
    }

    pub fn detect(&mut self, sample_rate: f32, buffer: &[f32]) {
        if sample_rate == 0.0 {
            return;
        }

        let time = Instant::now();

        let result = self
            .detector
            .get_pitch(buffer, sample_rate as usize, 1.0, 0.4);

        if let Some(pitch) = result {
            futures::executor::block_on(async {
                let mut m = self.model.lock().await;
                m.add_measure(time, pitch.frequency);
            });
        }
    }
}
