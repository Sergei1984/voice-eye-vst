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
}
