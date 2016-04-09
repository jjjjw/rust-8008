//! An example of using dsp-chain's `Graph` type to create a simple Synthesiser with 3 sine wave
//! oscillators.

extern crate dsp;
extern crate portaudio;
extern crate rand;
extern crate utils;
mod waveforms;

use dsp::{sample, Graph, Node, Settings};
use portaudio as pa;
use waveforms::{Waveform};
use std::f32::consts::PI;

/// SoundStream is currently generic over i8, i32 and f32. Feel free to change it!
type Output = f32;

type Phase = f64;
type Frequency = f64;
type Volume = f32;
type Cutoff = f32;

const CHANNELS: i32 = 2;
const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;

const A3_HZ: Frequency = 220.0;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), pa::Error> {

    // Construct our dsp graph.
    let mut graph = Graph::new();

    // Construct our fancy Synth and add it to the graph!
    let synth = graph.add_node(DspNode::Synth);

    // Connect a filter before output
    let filter = graph.add_input(DspNode::LPFilter(8000f32), synth);
    // Connect an Oscillator to generate sound
    graph.add_input(DspNode::Oscillator(0.0, A3_HZ), filter);
    // graph.add_input(DspNode::Oscillator(0.0, D5_HZ, 0.1), synth);
    // graph.add_input(DspNode::Oscillator(0.0, F5_HZ, 0.15), synth);

    // Set the synth as the master node for the graph.
    graph.set_master(Some(synth));

    // We'll use this to count down from one second and then break from the loop.
    let mut timer: f64 = 1.0;

    // This will be used to determine the delta time between calls to the callback.
    let mut prev_time = None;

    // The callback we'll use to pass to the Stream. It will request audio from our dsp_graph.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, time, .. }| {

        sample::buffer::equilibrium(buffer);
        let settings = Settings::new(SAMPLE_HZ as u32, frames as u16, CHANNELS as u16);
        graph.audio_requested(buffer, settings);

        let last_time = prev_time.unwrap_or(time.current);
        let dt = time.current - last_time;
        timer -= dt;
        prev_time = Some(time.current);


        if timer >= 0.0 { pa::Continue } else { pa::Complete }
    };

    // Construct PortAudio and the stream.
    let pa = try!(pa::PortAudio::new());
    let settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_HZ, FRAMES));
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    // Wait for our stream to finish.
    while let true = try!(stream.is_active()) {
        ::std::thread::sleep(::std::time::Duration::from_millis(16));
    }

    Ok(())
}

/// Our type for which we will implement the `Dsp` trait.
#[derive(Debug)]
enum DspNode {
    /// Synth will be our demonstration of a master GraphNode.
    Synth,
    /// Oscillator will be our generator type of node, meaning that we will override
    /// the way it provides audio via its `audio_requested` method.
    Oscillator(Phase, Frequency),
    LPFilter(Cutoff)
}

impl Node<Output> for DspNode {
    /// Here we'll override the audio_requested method and generate a sine wave.
    fn audio_requested(&mut self, buffer: &mut [Output], settings: Settings) {
        match *self {
            DspNode::Synth => (),
            DspNode::Oscillator(ref mut phase, frequency) => {
                for frame in buffer.chunks_mut(settings.channels as usize) {
                    let val = waveforms::Saw.amp_at_phase(*phase);
                    for channel in frame.iter_mut() {
                        *channel = val;
                    }
                    *phase += frequency / settings.sample_hz as f64;
                }
            },
            DspNode::LPFilter(cutoff) => {
                for frame in buffer.chunks_mut(settings.channels as usize) {
                    for (i,channel) in frame.iter_mut().enumerate() {
                        // Calculate
                        let K = (PI * cutoff / (settings.sample_hz as f32)).tan();
                        let alpha = ((K-1.0) / (K+1.0), 0.0);
                        let y1 = alpha*channel + self.x_last[i] - alpha*self.y1_last[i];
                        let y = (channel+y1)/2.0;
                        *channel = y;
                        // Store our results
                        // self.x_last[i] = *channel;
                        // self.y1_last[i] = y1;
                    }
                }
            }
        }
    }
}
