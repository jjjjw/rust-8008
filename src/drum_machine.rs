use sample::Frame;
use sample::frame::Stereo;

pub type Velocity = f32;
pub type PadNum = usize;
pub type SampleHz = f64;

pub type AudioOut = Stereo<f32>;

/// A single drum pad that can be triggered with a velocity
struct Pad {
    /// Is the pad active
    is_active: bool,
    /// The current velocity
    vel: Velocity,
}

/// Create a new vector of pads
fn init_pads(num_pads: PadNum) -> Vec<Pad> {
    (0..num_pads)
        .map(|_| {
            Pad {
                is_active: false,
                vel: 0.0,
            }
        })
        .collect()
}

pub struct Machine {
    /// Amplitude multiplier (volume).
    pub volume: f32,
    /// Render playback or not.
    pub is_paused: bool,
    /// Number of triggerable pads.
    pub num_pads: PadNum,
    /// The pad states.
    pads: Vec<Pad>,
}

impl Machine {
    /// Constructor for a new drum machine.
    pub fn new() -> Self {
        let num_pads: PadNum = 2;
        Machine {
            volume: 1.0,
            is_paused: false,
            num_pads: num_pads,
            pads: init_pads(num_pads),
        }
    }

    /// Trigger a pad.
    pub fn pad_on<T>(&mut self, pad: PadNum, vel: Velocity) {
        if pad < self.num_pads {
            self.pads[pad].is_active = true;
            self.pads[pad].vel = vel;
        }
    }

    /// Deactivate a pad.
    pub fn pad_off<T>(&mut self, pad: PadNum) {
        if pad < self.num_pads {
            self.pads[pad].is_active = false;
            self.pads[pad].vel = 0.0;
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
            pad.is_active = false;
            pad.vel = 0.0;
        }
    }

    /// Output a vector of audio frames
    pub fn render(&mut self, sample_rate: SampleHz) -> Vec<AudioOut> {
        if self.is_paused {
            vec![AudioOut::equilibrium()]
        } else {
            // TODO: render all active pads
            vec![AudioOut::equilibrium()]
        }
    }
}
