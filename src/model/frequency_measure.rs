use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
pub struct FrequencyMeasure {
    pub frequency: f32,
    pub time: Instant,
}
