use std::collections::VecDeque;

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
}
