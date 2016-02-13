extern crate dsp;
extern crate pitch_calc as pitch;
extern crate portaudio;
extern crate synth;
extern crate time_calc as time;

use dsp::{Node, Sample, Settings};
use portaudio as pa;
use pitch::{Letter, LetterOctave};
use synth::Synth;

// Currently supports i8, i32, f32.
pub type AudioSample = f32;
pub type Input = AudioSample;
pub type Output = AudioSample;

const CHANNELS: i32 = 2;
const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;

fn main() {
    println!("Hello, world!");
}
