use std::time::{Duration, Instant};

use super::FrequencyMeasure;

pub(crate) struct FrequencyBucket {
    time: Instant,
    buffer: Vec<FrequencyMeasure>,
}

impl FrequencyBucket {
    pub fn new() -> Self {
        FrequencyBucket {
            time: Instant::now(),
            buffer: Vec::new(),
        }
    }

    pub fn push_measure(&mut self, measure: FrequencyMeasure) {
        self.buffer.push(measure);
    }

    pub fn duration(&self) -> Duration {
        if let Some(last) = self.buffer.get(self.buffer.len() - 1) {
            return last.time.duration_since(self.time);
        }

        Duration::default()
    }

    pub fn started(&self) -> Instant {
        self.time
    }
}
