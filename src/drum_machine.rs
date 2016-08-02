
pub struct Machine {
    /// Amplitude multiplier (volume).
    pub volume: f32,
}

impl Machine {
    /// Constructor for a new drum machine.
    pub fn new() -> Self {
        Machine { volume: 1.0 }
    }
}
