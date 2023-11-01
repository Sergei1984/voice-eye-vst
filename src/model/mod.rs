use std::{
    collections::VecDeque,
    mem,
    time::{Duration, Instant},
};

use self::{frequency_bucket::FrequencyBucket, frequency_measure::FrequencyMeasure};

mod frequency_bucket;
mod frequency_measure;
mod test;

const PITCH_RESOLUTION_MS: Duration = Duration::from_millis(100);
const BUCKET_LEN_MS: Duration = Duration::from_secs(5);

pub struct MeasureModel {
    /// Stores average pitch frequency with 1/10 seconds resolution
    buckets: VecDeque<FrequencyBucket>,

    /// The moment of time when started accumulated pitches in avg_buffer
    avg_time: Instant,

    /// The pitches accumulated due the PITCH_RESOLUTION_MS period
    avg_buffer: Vec<f32>,

    current_filling_bucket: FrequencyBucket,
}

/// Contains a sequential frequencies for a time span of max BUCKET_LEN_MS length

impl MeasureModel {
    pub fn new() -> Self {
        let mut result = MeasureModel {
            buckets: VecDeque::new(),
            avg_time: Instant::now(),
            avg_buffer: Vec::<f32>::new(),
            current_filling_bucket: FrequencyBucket::new(),
        };

        result.avg_buffer.reserve(44100 / 512);

        result
    }

    pub fn add_measure(&mut self, time: Instant, frequency: f32) {
        if time.duration_since(self.current_filling_bucket.started()) > BUCKET_LEN_MS {
            let new_bucket = FrequencyBucket::new();

            let old_current_bucket = mem::replace(&mut self.current_filling_bucket, new_bucket);
            self.buckets.push_back(old_current_bucket);
        }

        if (self.avg_time - time) > PITCH_RESOLUTION_MS {
            let avg = self.avg_buffer.iter().sum::<f32>() / (self.avg_buffer.len() as f32);

            self.current_filling_bucket.push_measure(FrequencyMeasure {
                time: self.avg_time,
                frequency: avg,
            });

            self.avg_time = Instant::now();
            self.avg_buffer.clear();
        }

        self.avg_buffer.push(frequency);
    }

    // /// Iterates frequency measures starting from the newest
    // /// until total length of consequent parts less than specified duration
    // pub fn latest(&self, total_duration: Duration) -> impl Iterator<Item = &FrequencyBucket> + '_ {
    //     todo!()
    // }
}
