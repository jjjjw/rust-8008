//!
//! The Waveform trait along with various Waveform Types and there implementations.
//!

/// Some type that can return an amplitude given some phase.
pub trait Waveform {
    /// Return the amplitude given some phase.
    fn amp_at_phase(&self, phase: f64) -> f32;
    /// An optional method for processing the frequency.
    #[inline]
    fn process_hz(&self, hz: f64) -> f64 { hz }
}


/// Twice PI.
const PI_2: f64 = ::std::f64::consts::PI * 2.0;

/// Represents the "steepness" of the exponential saw wave.
pub type Steepness = f32;

/// A sine wave.
#[derive(Copy, Clone, Debug)]
pub struct Sine;

/// A sawtooth wave.
#[derive(Copy, Clone, Debug)]
pub struct Saw;

/// An exponential sawtooth wave.
#[derive(Copy, Clone, Debug)]
pub struct SawExp(pub Steepness);

/// A square wave.
#[derive(Copy, Clone, Debug)]
pub struct Square;

/// A noise signal.
#[derive(Copy, Clone, Debug)]
pub struct Noise;


impl Waveform for Sine {
    #[inline]
    fn amp_at_phase(&self, phase: f64) -> f32 {
        (PI_2 * phase).sin() as f32
    }
}

impl Waveform for Saw {
    #[inline]
    fn amp_at_phase(&self, phase: f64) -> f32 {
        (::utils::fmod(phase, 1.0) * -2.0 + 1.0) as f32
    }
}

impl Waveform for SawExp {
    #[inline]
    fn amp_at_phase(&self, phase: f64) -> f32 {
        let SawExp(steepness) = *self;
        let saw = Saw.amp_at_phase(phase);
        saw * saw.abs().powf(steepness)
    }
}

impl Waveform for Square {
    #[inline]
    fn amp_at_phase(&self, phase: f64) -> f32 {
        (if ::utils::fmod(phase, 1.0) < 0.5 { -1.0 } else { 1.0 }) as f32
        //(if (PI_2 * phase).sin() < 0.0 { -1.0 } else { 1.0 }) as f32
    }
}

impl Waveform for Noise {
    #[inline]
    fn amp_at_phase(&self, _phase: f64) -> f32 {
        ::rand::random::<f32>() * 2.0 - 1.0
    }
}
