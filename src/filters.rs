// copied from https://github.com/oxcable/oxcable/blob/master/src/filters/first_order.rs

use std::f64::consts::PI;
use num::traits::Float;
use dsp::{Settings};

pub fn decibel_to_ratio(db: f64) -> f64 {
    10.0f64.powf(db/10.0)
}

pub enum FilterMode {
    /// LowPass(cutoff)
    LowPass(f64),
    /// HighPass(cutoff)
    HighPass(f64),
    /// LowShelf(cutoff, gain)
    LowShelf(f64, f64),
    /// HighShelf(cutoff, gain)
    HighShelf(f64, f64)
}
use self::FilterMode::*;

pub struct Filter {
    x_last: f64,
    y1_last: f64,
    mode: FilterMode
}

impl Filter {
    /// Creates a new first order filter with the provided mode. Each channel is
    /// filtered independently.
    pub fn new(mode: FilterMode) -> Self {
        Filter {
            x_last: 0.0,
            y1_last: 0.0,
            mode: mode
        }
    }

    pub fn tick(&mut self, amp: f64, settings: Settings) -> f64 {
        // Run the all pass filter, and feedback the result
        let (alpha, H0) = self.compute_parameters(settings);
        let x = amp;
        let y1 = alpha * x + self.x_last - alpha * self.y1_last;
        let y = match self.mode {
            LowPass(_) => (x+y1)/2.0,
            HighPass(_) => (x-y1)/2.0,
            LowShelf(_,_) => H0*(x+y1)/2.0 + x,
            HighShelf(_,_) => H0*(x-y1)/2.0 + x
        };

        // Store our results
        self.x_last = x;
        self.y1_last = y1;
        y
    }

    /// Computes the (alpha, H0) parameters for our filter
    fn compute_parameters(&self, settings: Settings) -> (f64, f64) {
        let cutoff = match self.mode {
            LowPass(cutoff) => cutoff,
            HighPass(cutoff) => cutoff,
            LowShelf(cutoff, _) => cutoff,
            HighShelf(cutoff, _) => cutoff
        };
        let K = (PI * cutoff / (settings.sample_hz as f64)).tan();

        match self.mode {
            LowPass(_) | HighPass(_) => {
                ((K-1.0) / (K+1.0), 0.0)
            },
            LowShelf(_, gain) => {
                let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
                let H0 = V0 - 1.0;
                let alpha = if gain < 0.0 {
                    (K-V0) / (K+V0)
                } else {
                    (K-1.0) / (K+1.0)
                };
                (alpha, H0)
            },
            HighShelf(_, gain) => {
                let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
                let H0 = V0 - 1.0;
                let alpha = if gain > 0.0 {
                    (V0*K-1.0) / (K+1.0)
                } else {
                    (K-1.0) / (K+1.0)
                };
                (alpha, H0)
            }
        }
    }
}
