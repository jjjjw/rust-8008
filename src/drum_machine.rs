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
    /// The sound generator for this pad.
    gen: Generator,
}

/// Return the sine wave at the phase (TODO: waveforms module)
const PI_2: f64 = ::std::f64::consts::PI * 2.0;
fn sine(phase: f64) -> f64 {
    (PI_2 * phase).sin()
}

/// An audio generator
struct Generator {
    /// The duration of the audio.
    duration: Ms,
    /// The amplitude envelope of this sound.
    amp_env: Envelope,
    /// Oscillator state (TODO: OscState struct)
    phase: f64,
    /// Oscillator state (TODO: OscState struct)
    frequency: f64,
}

impl Generator {
    fn new() -> Self {
        let amp_env = Envelope::from(vec!(
            //         Time ,  Amp ,  Curve
            Point::new(0.0  ,  0.1 ,  0.0),
            Point::new(0.01 ,  1.0 ,  0.0),
            Point::new(0.25 ,  0.8 ,  0.0),
            Point::new(0.75 ,  0.2 ,  0.0),
            Point::new(1.0  ,  0.0 ,  0.0),
        ));

        Generator {
            duration: Ms(1.0),
            amp_env: amp_env,
            phase: 0.0,
            frequency: 32_700.0,
        }
    }

    /// Get the next phase
    fn next_phase(&mut self, sample_hz: SampleHz) -> f64 {
        self.phase += self.frequency / sample_hz;
        self.phase
    }

    /// Get the next amplitude multiplier
    fn next_amp_mul(&mut self, active_time: Ms) -> Option<f64> {
        let perc = active_time.ms() / self.duration.ms();
        self.amp_env.y(perc)
    }

    /// Get the next amplitude
    fn next_amp(&mut self, sample_hz: SampleHz, active_time: Ms) -> Option<f64> {
        let mul = self.next_amp_mul(active_time);
        match mul {
            Some(val) => Some(sine(self.next_phase(sample_hz)) * val),
            None => None,
        }
    }
}

impl PadState {
    fn new() -> Self {
        PadState {
            active: false,
            active_time: Ms(0.0),
            vel: 0.0,
            gen: Generator::new(),
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

    /// Get the next audio frame.
    fn next_frame(&mut self, sample_hz: SampleHz) -> AudioOut {
        if !self.active {
            AudioOut::equilibrium()
        } else {
            // Step forward by the sample rate
            self.active_time = self.active_time + Samples(1).to_ms(sample_hz);
            let next = self.gen.next_amp(sample_hz, self.active_time);
            match next {
                Some(val) => [val].scale_amp(self.vel),
                None => {
                    self.silence();
                    AudioOut::equilibrium()
                }
            }
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
