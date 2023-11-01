use std::time::{Duration, Instant};

use super::{FrequencyMeasure, PITCH_RESOLUTION};

pub struct FrequencyBucket {
    time: Instant,
    buffer: Vec<FrequencyMeasure>,

    /// The moment of time when started accumulated pitches in avg_buffer
    avg_time: Instant,

    /// The pitches accumulated due the PITCH_RESOLUTION_MS period
    avg_buffer: Vec<f32>,
}

impl FrequencyBucket {
    pub fn new() -> Self {
        let mut result = FrequencyBucket {
            time: Instant::now(),
            buffer: Vec::new(),
            avg_time: Instant::now(),
            avg_buffer: Vec::<f32>::new(),
        };

        result.avg_buffer.reserve(44100 / 512);

        result
    }

    pub fn measures(&self) -> impl Iterator<Item = &FrequencyMeasure> + '_ {
        self.buffer.iter()
    }

    pub fn push_measure(&mut self, time: Instant, frequency: f32) {
        if !self.avg_buffer.is_empty() && (time - self.avg_time) > PITCH_RESOLUTION {
            self.flush_aggregation();
            self.avg_buffer.clear();
        }

        if self.avg_buffer.is_empty() {
            self.avg_time = time;
        }

        self.avg_buffer.push(frequency);
    }

    pub(crate) fn flush_aggregation(&mut self) {
        if !self.avg_buffer.is_empty() {
            let avg = self.avg_buffer.iter().sum::<f32>() / (self.avg_buffer.len() as f32);

            self.buffer.push(FrequencyMeasure {
                time: self.avg_time,
                frequency: avg,
            });
        }
    }

    pub fn duration(&self) -> Duration {
        if self.buffer.is_empty() {
            return Duration::default();
        }

        if let Some(last) = self.buffer.get(self.buffer.len() - 1) {
            return last.time.duration_since(self.time);
        }

        Duration::default()
    }

    pub fn started(&self) -> Instant {
        self.time
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
