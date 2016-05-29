use dsp::{Node, Settings};
use waveforms::{Waveform, Sine};

pub type Frequency = f64;
pub type Phase = f64;
pub type Output = f32; // f64 not supported yet
pub type Amp = f64;
pub type Volume = f64;

pub struct Oscillator {
  phase: Phase,
  waveform: Sine // TODO: any waveform
}

pub struct Synth {
  oscillator: Oscillator,
  frequency: Frequency,
  volume: Amp,
}

impl Oscillator {
    pub fn new(waveform: Sine) -> Oscillator {
        Oscillator {
            phase: 0.0,
            waveform: waveform
        }
    }

    pub fn amp_at_next_phase(&mut self, frequency: Frequency, settings: Settings) -> Amp {
        let val = self.waveform.amp_at_phase(self.phase);
        self.phase += frequency / settings.sample_hz as f64;
        val
    }
}

impl Synth {
    pub fn new(oscillator: Oscillator, frequency: Frequency, volume: Amp) -> Synth {
        Synth {
            oscillator: oscillator, // TODO: multiple oscillator
            frequency: frequency,  // TODO: dynamic frequency
            volume: volume  // TODO: dynamic volume
        }
    }
}

impl Node<Output> for Synth {
    /// Here we'll override the audio_requested method and generate a sound.
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        for frame in buffer.chunks_mut(settings.channels as usize) {
            let amp = self.oscillator.amp_at_next_phase(self.frequency, settings);
            for channel in frame.iter_mut() {
                *channel = (amp * self.volume) as f32;
            }
        }
    }
}

pub fn sine_synth (frequency: Frequency) -> Synth {
    Synth::new(Oscillator::new(Sine), frequency, 0.15)
}
