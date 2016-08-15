use dsp::Node;
use dsp::sample::frame::Stereo;
use dsp::sample::signal;
use dsp::sample::{Frame, Signal, slice};

pub type Hz = u64;
pub type PadIdx = usize;
pub type Velocity = u64;
pub type Volume = u64;
pub type AudioOut = Stereo<f64>;

struct PadState {
    /// Is the pad active.
    active: bool,
    /// The velocity this pad was triggered with.
    vel: Velocity,
    /// The current sample rate
    sample_hz: Hz,
}

impl PadState {
    fn new(sample_hz: Hz) -> Self {
        PadState {
            active: false,
            vel: 0,
            sample_hz: sample_hz,
        }
    }

    /// Trigger a sound.
    fn trigger(&mut self, vel: Velocity) {
        self.active = true;
        self.vel = vel;
    }

    /// Deactivate.
    fn silence(&mut self) {
        self.active = false;
        self.vel = 0;
    }

    /// Set the sample rate
    fn set_sample_rate(&mut self, sample_hz: Hz) {
        self.sample_hz = sample_hz;
    }

    /// Get the next audio frame.
    fn next_frame(&mut self) -> AudioOut {
        AudioOut::equilibrium()
    }
}

impl Iterator for PadState {
    type Item = AudioOut;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_frame())
    }
}

pub struct Machine {
    /// Amplitude multiplier (volume).
    pub volume: Volume,
    /// Render playback or not.
    pub is_paused: bool,
    /// The current sample rate
    sample_hz: Hz,
    /// The states of the pads that generate sound for the machine.
    pads: Vec<PadState>,
}

impl Machine {
    /// Constructor for a new drum machine.
    pub fn new() -> Self {
        const sample_hz: Hz = 44_100;
        Machine {
            volume: 1,
            is_paused: false,
            pads: vec![PadState::new(sample_hz)],
            sample_hz: sample_hz,
        }
    }

    /// Trigger a pad.
    pub fn trigger(&mut self, pad: PadIdx, vel: Velocity) {
        if pad < self.pads.len() {
            self.pads[pad].trigger(vel);
        }
    }

    /// Deactivate a pad.
    pub fn silence(&mut self, pad: PadIdx) {
        if pad < self.pads.len() {
            self.pads[pad].silence();
        }
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Unpause playback.
    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    /// Stop playback and clear the current pads.
    pub fn stop(&mut self) {
        for pad in &mut self.pads {
            pad.silence();
        }
    }

    /// Set the sample rate
    pub fn set_sample_rate(&mut self, sample_hz: Hz) {
        self.sample_hz = sample_hz;
        for pad in &mut self.pads {
            pad.set_sample_rate(sample_hz);
        }
    }

    /// Get the next audio frame.
    fn next_frame(&mut self) -> AudioOut {
        self.pads.iter_mut().fold(AudioOut::equilibrium(),
                                  |f, pad| f.add_amp(pad.next_frame()))
    }
}

impl Iterator for Machine {
    type Item = AudioOut;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_frame())
    }
}

impl Node<AudioOut> for Machine {
    fn audio_requested(&mut self, buffer: &mut [AudioOut], sample_hz: f64) {
        let sample_hz_rounded = sample_hz.round() as u64;
        if sample_hz_rounded != self.sample_hz {
            self.set_sample_rate(sample_hz_rounded);
        }

        slice::map_in_place(buffer, |_| match self.next() {
            None => AudioOut::equilibrium(),
            Some(output) => output,
        });
    }
}
