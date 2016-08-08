use dsp::sample::frame::Stereo;
use dsp::sample::Frame;
use dsp::{Graph, Node};

pub type PadNum = usize;
pub type Velocity = f32;
pub type Hz = f64;

pub type Output = Stereo<f32>;

struct EmptyNode;

impl Node<Output> for EmptyNode {
    fn audio_requested(&mut self, buffer: &mut [Output], sample_hz: Hz) {}
}

pub struct Machine {
    /// Amplitude multiplier (volume).
    pub volume: f32,
    /// Render playback or not.
    pub is_paused: bool,
    /// The sample rate
    pub sample_hz: Hz,
    /// The audio dsp graph that generates audio.
    graph: Graph<Output, EmptyNode>,
}

impl Machine {
    /// Constructor for a new drum machine.
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let root = graph.add_node(EmptyNode);

        Machine {
            volume: 1.0,
            is_paused: false,
            sample_hz: 44_100.0,
            graph: graph,
        }
    }

    /// Trigger a pad.
    pub fn trigger<T>(&mut self, pad: PadNum, vel: Velocity) {}

    /// Deactivate a pad.
    pub fn silence<T>(&mut self, pad: PadNum) {}

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

    /// Get from the graph at the sample rate.
    fn audio_requested(&mut self) -> Vec<Output> {
        let mut buffer = vec![Output::equilibrium()];
        if self.is_paused {
            buffer
        } else {
            self.graph.audio_requested(&mut buffer, self.sample_hz);
            // TODO: volume
            buffer
        }
    }
}
