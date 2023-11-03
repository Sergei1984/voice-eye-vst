use std::{
    collections::VecDeque,
    mem,
    time::{Duration, Instant},
};

use self::{frequency_bucket::FrequencyBucket, frequency_measure::FrequencyMeasure};

mod frequency_bucket;
mod frequency_measure;
mod test;

const PITCH_RESOLUTION: Duration = Duration::from_millis(100);
const BUCKET_LEN: Duration = Duration::from_secs(5);

pub struct MeasureModel {
    /// Stores average pitch frequency with 1/10 seconds resolution
    buckets: VecDeque<FrequencyBucket>,
    current_filling_bucket: FrequencyBucket,
}

/// Contains a sequential frequencies for a time span of max BUCKET_LEN_MS length

impl MeasureModel {
    pub fn new() -> Self {
        MeasureModel {
            buckets: VecDeque::new(),
            current_filling_bucket: FrequencyBucket::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.buckets.len()
    }

    pub fn add_measure(&mut self, time: Instant, frequency: f32) {
        if time.duration_since(self.current_filling_bucket.started()) > BUCKET_LEN {
            let new_bucket = FrequencyBucket::new();

            let mut old_current_bucket = mem::replace(&mut self.current_filling_bucket, new_bucket);
            old_current_bucket.flush_aggregation();

            if old_current_bucket.len() > 0 {
                self.buckets.push_back(old_current_bucket);
            }
        }

        self.current_filling_bucket.push_measure(time, frequency);
    }

    pub fn current<'a>(&'a self) -> &'a FrequencyBucket {
        &self.current_filling_bucket
    }

    /// Iterates frequency measures starting from the newest
    /// until total length of consequent parts less than specified duration
    pub fn latest(&self, total_duration: Duration) -> impl Iterator<Item = &FrequencyBucket> + '_ {
        let mut remaining = total_duration - self.current().duration();

        std::iter::once(self.current()).chain(self.buckets.iter().rev().take_while(move |b| {
            if remaining.is_zero() {
                false
            } else {
                remaining = if remaining < b.duration() {
                    Duration::default()
                } else {
                    remaining - b.duration()
                };
                true
            }
        }))
    }
}
