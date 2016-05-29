use dsp::{Node, Settings};
use waveforms::{Waveform, amp_at_phase, Phase, Amp};

pub type Frequency = f64;
pub type Output = f32; // f64 not supported yet
pub type Volume = f64;

pub struct Oscillator {
  phase: Phase,
  waveform: Waveform
}

pub struct Synth {
  frequency: Frequency,
  volume: Amp,
  oscillator: Oscillator,
}

impl Oscillator {
    pub fn new(waveform: Waveform) -> Oscillator {
        Oscillator {
            phase: 0.0,
            waveform: waveform
        }
    }

    pub fn tick(&mut self, frequency: Frequency, settings: Settings) -> Amp {
        let amp = amp_at_phase(&self.waveform, self.phase);
        self.phase += frequency / settings.sample_hz as f64;
        amp
    }
}

impl Synth {
    pub fn new(oscillator: Oscillator, frequency: Frequency, volume: Amp) -> Synth {
        Synth {
            oscillator: oscillator, // TODO: multiple.dynamic oscillators
            frequency: frequency,  // TODO: dynamic frequency
            volume: volume  // TODO: dynamic volume
        }
    }
}

impl Node<Output> for Synth {
    /// Here we'll override the audio_requested method and generate a sound.
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        for frame in buffer.chunks_mut(settings.channels as usize) {
            let amp = self.oscillator.tick(self.frequency, settings);
            for channel in frame.iter_mut() {
                *channel = (amp * self.volume) as f32;
            }
        }
    }
}

pub fn sine_synth (frequency: Frequency) -> Synth {
    Synth::new(Oscillator::new(Waveform::Square), frequency, 0.15)
}
