#[cfg(test)]
mod test {
    use crate::music::notes::{Frequency, Note, Octave};

    #[test]
    fn frequency_of() {
        assert_eq!(16.35, Frequency::of(Octave::SubContra, Note::C));
        assert_eq!(30.87, Frequency::of(Octave::SubContra, Note::B));
        assert_eq!(440.0, Frequency::of(Octave::First, Note::A));
        assert_eq!(7902.13, Frequency::of(Octave::Fifth, Note::B));
    }

    #[test]
    fn project() {
        let low = 220.0;
        let high = low * 2.0 * 2.0 * 2.0 * 2.0;

        let height = 1000;

        let a3_height = Frequency::project(220.0, low, high, height);
        let a4_height = Frequency::project(440.0, low, high, height);
        let a5_height = Frequency::project(880.0, low, high, height);
        let a6_height = Frequency::project(1760.0, low, high, height);
        let a7_height = Frequency::project(3520.0, low, high, height);

        println!(
            "A3={} A4={} A5={} A6={} A7={}",
            a3_height, a4_height, a5_height, a6_height, a7_height
        );

        assert!(a3_height.abs_diff(0) < 2);
        assert!(a4_height.abs_diff(250) < 2);
        assert!(a5_height.abs_diff(500) < 2);
        assert!(a6_height.abs_diff(750) < 2);
        assert!(a7_height.abs_diff(1000) < 2);

        assert!(false);
    }
}
