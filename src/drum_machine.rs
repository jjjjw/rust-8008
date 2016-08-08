use dsp::sample::Frame;
use dsp::Node;

pub type Hz = f64;
pub type Volume = f64;
pub type Velocity = f64;

enum Pad {
    Input1,
}

pub struct Machine {
    /// Amplitude multiplier (volume).
    pub volume: Volume,
    /// Render playback or not.
    pub is_paused: bool,
    /// The sample rate
    pub sample_hz: Hz,
}

impl Machine {
    /// Constructor for a new drum machine.
    pub fn new() -> Self {
        Machine {
            volume: 1.0,
            is_paused: false,
            sample_hz: 44_100.0,
        }
    }

    /// Trigger a pad.
    pub fn trigger<T>(&mut self, pad: Pad, vel: Velocity) {}

    /// Deactivate a pad.
    pub fn silence<T>(&mut self, pad: Pad) {}

    /// Pause playback.
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Unpause playback.
    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    /// Stop playback and clear the current pads.
    pub fn stop(&mut self) {}
}

impl<F> Node<F> for Machine
    where F: Frame
{
    fn audio_requested(&mut self, buffer: &mut [F], sample_hz: Hz) {}
}
