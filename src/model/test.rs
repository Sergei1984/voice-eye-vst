#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use crate::model::MeasureModel;

    #[test]
    fn add_measure_works() {
        let mut model = MeasureModel::new();

        let start_time = Instant::now();

        model.add_measure(start_time, 200.0);
        model.add_measure(start_time + Duration::from_secs(10), 100.0);
        model.add_measure(start_time + Duration::from_secs(20), 50.0); // Last measure might not fill the buffer and stay in current bucket

        assert_eq!(model.len(), 2);
    }

    #[test]
    fn average_aggregate_measure_works() {
        let mut model = MeasureModel::new();

        let start_time = Instant::now();

        model.add_measure(start_time + Duration::default(), 200.0);
        model.add_measure(start_time + Duration::from_millis(10), 220.0);
        model.add_measure(start_time + Duration::from_millis(20), 240.0);
        model.add_measure(start_time + Duration::from_millis(30), 250.0);
        model.add_measure(start_time + Duration::from_millis(40), 120.0);

        model.current_filling_bucket.flush_aggregation();

        assert_eq!(model.current().len(), 1);

        let measure = model.current().measures().next().unwrap();
        assert_eq!(measure.time, start_time);
        assert_eq!(
            measure.frequency,
            (200.0 + 220.0 + 240.0 + 250.0 + 120.0) / 5.0
        );
    }
}
