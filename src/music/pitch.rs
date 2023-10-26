use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::time::Instant;

pub struct MyPitchDetector {
    last_freq: f32,
    file: File,
    detector: McLeodDetector<f32>,
    time: Instant,
}

impl MyPitchDetector {
    pub fn new() -> Self {
        MyPitchDetector {
            last_freq: 0.0,
            file: OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("/Users/sergiitokariev/Downloads/voice-eye-log.txt")
                .unwrap(),
            detector: McLeodDetector::new(512, 256),
            time: Instant::now(),
        }
    }

    pub fn detect(&mut self, sample_rate: f32, buffer: &[f32]) {
        if sample_rate == 0.0 {
            return;
        }

        let result = self
            .detector
            .get_pitch(buffer, sample_rate as usize, 1.0, 0.4);

        if let Some(pitch) = result {
            if (self.last_freq - pitch.frequency).abs() > 0.1 {
                self.last_freq = pitch.frequency;
                let _ = writeln!(self.file, "{:?}: {}", self.time.elapsed(), self.last_freq);
                let _ = self.file.flush();
            }
        }
    }
}
