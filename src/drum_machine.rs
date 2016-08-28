use dsp::Node;
use dsp::sample::frame::Mono;
use dsp::sample::{Frame, slice};
use envelope::{Envelope, Point};
use envelope_lib::Envelope as Trait;
use time_calc::{SampleHz, Ms, Samples};
use std::f64;

pub type PadIdx = usize;
pub type Velocity = f64;
pub type Volume = f64;
pub type AudioOut = Mono<f64>;

struct PadState {
    /// Is the pad active.
    active: bool,
    /// The time that the pad has been active.
    active_time: Ms,
    /// The velocity this pad was triggered with.
    vel: Velocity,
    /// The duration of the active state.
    duration: Ms,
    /// The amplitude envelope of this pad.
    amp_env: Envelope,
}

impl PadState {
    fn new() -> Self {
        let amp_env = Envelope::from(vec!(
            //         Time ,  Amp ,  Curve
            Point::new(0.0  ,  0.1 ,  0.0),
            Point::new(0.01 ,  1.0 ,  0.0),
            Point::new(0.25 ,  0.8 ,  0.0),
            Point::new(0.75 ,  0.2 ,  0.0),
            Point::new(1.0  ,  0.0 ,  0.0),
        ));

        PadState {
            active: false,
            active_time: Ms(0.0),
            vel: 0.0,
            duration: Ms(1_000.0),
            amp_env: amp_env,
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
        self.active_time = Ms(0.0);
        self.vel = 0.0;
    }

    /// Get the next amplitude multiplier
    fn next_amp(&mut self, sample_hz: SampleHz) -> f64 {
        // Step forward by the sample rate
        self.active_time = self.active_time + Samples(1).to_ms(sample_hz);
        let perc = self.active_time.ms() / self.duration.ms();

        if perc > 1.0 {
            self.silence();
            0.0
        } else {
            self.amp_env.y(perc).unwrap()
        }
    }

    /// Get the next audio frame.
    fn next_frame(&mut self, sample_hz: SampleHz) -> AudioOut {
        if !self.active {
            AudioOut::equilibrium()
        } else {
            // TODO: sound generator
            [1.0 * self.vel * self.next_amp(sample_hz)]
        }
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
    pub fn new() -> Self {;
        Machine {
            volume: 1.0,
            is_paused: false,
            pads: vec![PadState::new()],
        }
    }

    /// Trigger a pad.
    pub fn trigger(&mut self, pad: PadIdx, vel: Velocity) {
        if pad <= self.pads.len() {
            self.pads[pad - 1].trigger(vel);
        }
    }

    /// Deactivate a pad.
    pub fn silence(&mut self, pad: PadIdx) {
        if pad <= self.pads.len() {
            self.pads[pad - 1].silence();
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
    pub fn next_frame(&mut self, sample_hz: SampleHz) -> AudioOut {
        if self.is_paused {
            AudioOut::equilibrium()
        } else {
            self.pads
                .iter_mut()
                .fold(AudioOut::equilibrium(),
                      |f, pad| f.add_amp(pad.next_frame(sample_hz)))
                .scale_amp(self.volume)
        }
    }
}

impl Node<AudioOut> for Machine {
    fn audio_requested(&mut self, buffer: &mut [AudioOut], sample_hz: SampleHz) {
        slice::map_in_place(buffer, |_| self.next_frame(sample_hz));
    }
}
