use dsp::{Node, Settings};
use waveforms::{Waveform, Square};

pub type Frequency = f64;
pub type Phase = f64;
pub type Output = f64;

pub struct Oscillator {
  phase: Phase,
  waveform: Square // TODO: any waveform
}

pub struct Synth {
  oscillator: Oscillator,
  frequency: Frequency
}

impl Oscillator {
    pub fn new(waveform: Square) -> Oscillator {
        Oscillator {
            phase: 0.0,
            waveform: waveform
        }
    }

    pub fn amp_at_next_phase(&mut self, frequency: Frequency, settings: Settings) {
        let val = self.waveform.amp_at_phase(self.phase);
        self.phase = self.phase += frequency / settings.sample_hz as f64;
        val
    }
}

impl Synth {
    pub fn new(oscillator: Oscillator, frequency: Frequency) -> Synth {
        Synth {
            oscillator: oscillator, // TODO: multiple voices
            frequency: frequency  // TODO: dynamic frequency
        }
    }
}

impl Node<Output> for Synth {
    /// Here we'll override the audio_requested method and generate a sound.
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        for frame in buffer.chunks_mut(settings.channels as usize) {
            let val = self.oscillator.amp_at_next_phase(self.frequency, settings);
            for channel in frame.iter_mut() {
                *channel = val;
            }
        }
    }
}

pub fn SquareSynth (frequency: Frequency) -> Synth {
    Synth::new(Oscillator::new(Square), frequency)
}
