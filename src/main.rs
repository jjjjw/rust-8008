//! An example of using dsp-chain's `Graph` type to create a simple Synthesiser with 3 sine wave
//! oscillators.

extern crate dsp;
extern crate portaudio;
extern crate rand;
extern crate utils;
extern crate num;
mod waveforms;
mod synth;
mod filters;

use dsp::{sample, Node, Graph, Settings};
use portaudio as pa;
use synth::{square_synth, Frequency};

/// SoundStream is currently generic over i8, i32 and f32. Feel free to change it!
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
    let synth = graph.add_node(square_synth(A3_HZ));

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
