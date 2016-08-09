use dsp::Node;
use dsp::sample::frame::Stereo;
use dsp::sample::signal;
use dsp::sample::{Frame, Signal, slice};

pub type Hz = f64;
pub type PadIdx = usize;
pub type Velocity = f64;
pub type Volume = f64;
pub type AudioOut = Stereo<f64>;

struct PadState {
    /// Is the pad active.
    active: bool,
    /// The velocity this pad was triggered with.
    vel: Velocity,
}

impl PadState {
    fn new() -> Self {
        PadState {
            active: false,
            vel: 0.0,
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
        self.vel = 0.0;
    }

    /// Get the next audio frame.
    fn next_frame(&mut self, sample_hz: Hz) -> AudioOut {
        AudioOut::equilibrium()
    }
}

pub struct Machine {
    /// Amplitude multiplier (volume).
    pub volume: Volume,
    /// Render playback or not.
    pub is_paused: bool,
    /// The states of the pads that generate sound for the machine.
    pads: Vec<PadState>,
}

impl Machine {
    /// Constructor for a new drum machine.
    pub fn new() -> Self {
        Machine {
            volume: 1.0,
            is_paused: false,
            pads: vec![PadState::new()],
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

    /// Get the next audio frame.
    pub fn next_frame(&mut self, sample_hz: Hz) -> AudioOut {
        self.pads.iter_mut().fold(AudioOut::equilibrium(),
                                  |f, pad| f.add_amp(pad.next_frame(sample_hz)))
    }
}

impl Node<AudioOut> for Machine {
    fn audio_requested(&mut self, buffer: &mut [AudioOut], sample_hz: Hz) {
        slice::map_in_place(buffer, |_| self.next_frame(sample_hz));
    }
}
