#![allow(dead_code)]

// Note frequencies are from here
// https://mixbutton.com/mixing-articles/music-note-to-frequency-chart/

// Javascript snippet to get it from HTML
// (function() {
//    const notes = [...document.querySelectorAll("#tablepress-1 tr")];
//    const allFreqs = [];
//    for (let octave = 0; octave < 9; octave++) {
//        for (let noteNum = 1; noteNum < 13; noteNum++) {
//            const freqs = [...notes[noteNum].querySelectorAll("td")];
//            allFreqs.push(Number(freqs[octave + 1].textContent.replace(" Hz", "")));
//        }
//    }
//    console.log(JSON.stringify( allFreqs));
// })();

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Note {
    C = 0,
    CSharp = 1,
    D = 2,
    DSharp = 3,
    E = 4,
    F = 5,
    FSharp = 6,
    G = 7,
    GSharp = 8,
    A = 9,
    ASharp = 10,
    B = 11,
}

impl Note {
    pub fn all() -> Vec<Note> {
        vec![
            Self::C,
            Self::CSharp,
            Self::D,
            Self::DSharp,
            Self::E,
            Self::F,
            Self::FSharp,
            Self::G,
            Self::GSharp,
            Self::A,
            Self::ASharp,
            Self::B,
        ]
    }

    pub fn all_non_altered() -> Vec<Note> {
        vec![
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
            Self::A,
            Self::B,
        ]
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Octave {
    SubContra = 0,
    Contra = 1,
    Big = 2,
    Small = 3,
    First = 4,
    Second = 5,
    Third = 6,
    Fourth = 7,
    Fifth = 8,
}

pub struct Frequency;

impl Frequency {
    pub fn of(octave: Octave, note: Note) -> f32 {
        let o: usize = (octave as u8) as usize;
        let n: usize = (note as u8) as usize;
        NOTE_FREQUENCIES[o * 12 + n]
    }

    pub fn project(frequency: f32, low_frequency: f32, high_frequency: f32, heigh: u32) -> u32 {
        ((frequency / low_frequency).log2() * (heigh as f32)
            / (high_frequency / low_frequency).log2()) as u32
    }
}

const NOTE_FREQUENCIES: [f32; 108] = [
    16.35, 17.32, 18.35, 19.45, 20.6, 21.83, 23.12, 24.5, 25.96, 27.5, 29.14, 30.87, 32.7, 34.65,
    36.71, 38.89, 41.2, 43.65, 46.25, 49.0, 51.91, 55.0, 58.27, 61.74, 65.41, 69.3, 73.42, 77.78,
    82.41, 87.31, 92.5, 98.0, 103.83, 110.0, 116.54, 123.47, 130.81, 138.59, 146.83, 155.56,
    164.81, 174.61, 185.0, 196.0, 207.65, 220.0, 233.08, 246.94, 261.63, 277.18, 293.66, 311.13,
    329.63, 349.23, 369.99, 392.0, 415.3, 440.0, 466.16, 493.88, 523.25, 554.37, 587.33, 622.25,
    659.25, 698.46, 739.99, 783.99, 830.61, 880.0, 932.33, 987.77, 1046.5, 1108.73, 1174.66,
    1244.51, 1318.51, 1396.91, 1479.98, 1567.98, 1661.22, 1760.0, 1864.66, 1975.53, 2093.0,
    2217.46, 2349.32, 2489.02, 2637.02, 2793.83, 2959.96, 3135.96, 3322.44, 3520.0, 3729.31,
    3951.07, 4186.01, 4434.92, 4698.63, 4978.03, 5274.04, 5587.65, 5919.91, 6271.93, 6644.88,
    7040.0, 7458.62, 7902.13,
];
