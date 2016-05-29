use dsp::{Node, Settings};
use waveforms::{Waveform, amp_at_phase, Phase, Amp};
use filters::{FilterMode, Filter};

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
    filter: Filter,
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
    pub fn new(oscillator: Oscillator, filter: Filter, frequency: Frequency, volume: Amp) -> Synth {
        Synth {
            oscillator: oscillator, // TODO: multiple/dynamic oscillators
            filter: filter, // TODO: multiple/dynamic filters
            frequency: frequency,  // TODO: dynamic frequency
            volume: volume  // TODO: dynamic volume
        }
    }
}

impl Node<Output> for Synth {
    /// Here we'll override the audio_requested method and generate a sound.
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        for frame in buffer.chunks_mut(settings.channels as usize) {
            let mut amp = self.oscillator.tick(self.frequency, settings);
            amp = self.filter.tick(amp, settings);
            amp *= self.volume;
            for channel in frame.iter_mut() {
                *channel = amp as f32;
            }
        }
    }
}

pub fn square_synth (frequency: Frequency) -> Synth {
    let oscillator = Oscillator::new(Waveform::Square);
    let filter = Filter::new(FilterMode::LowPass(800f64));
    Synth::new(oscillator, filter, frequency, 0.15)
}
