use std::{
    collections::VecDeque,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Copy, Clone)]
pub struct FrequencyMeasure {
    pub frequency: f32,
    pub time: u128,
}

pub struct MeasureModel {
    measures: VecDeque<FrequencyMeasure>,
}

impl MeasureModel {
    pub fn new() -> Self {
        MeasureModel {
            measures: VecDeque::new(),
        }
    }

    pub fn add_measure(&mut self, time: u128, frequency: f32) {
        self.measures
            .push_back(FrequencyMeasure { time, frequency });
    }

    pub fn latest(&self, last_ms: u128) -> impl Iterator<Item = &FrequencyMeasure> + '_ {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            - last_ms;

        self.measures.iter().filter(move |m| m.time >= time)
    }
}
