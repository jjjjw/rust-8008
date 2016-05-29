const PI_2: f64 = ::std::f64::consts::PI * 2.0;

pub type Phase = f64;
pub type Amp = f64;

pub enum Waveform {
    Sine,
    Saw,
    Square,
    Noise
}

pub fn amp_at_phase(waveform: &Waveform, phase: Phase) -> Amp {
    match *waveform {
        Waveform::Sine => { (PI_2 * phase).sin() },
        Waveform::Saw => { ::utils::fmod(phase, 1.0) * -2.0 + 1.0 },
        Waveform::Square => { (if ::utils::fmod(phase, 1.0) < 0.5 { -1.0 } else { 1.0 }) },
        Waveform::Noise => { ::rand::random::<f64>() * 2.0 - 1.0 }
    }
}
