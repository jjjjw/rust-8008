#![feature(plugin)]
#![plugin(clippy)]
extern crate dsp;
extern crate envelope as envelope_lib;
extern crate time_calc;
extern crate pitch_calc;
mod drum_machine;
mod envelope;

/// Not doing anything yet, just getting started
fn main() {
    let mut machine = drum_machine::Machine::new();
    const SAMPLE_HZ: f64 = 44_100.0;
    machine.trigger(1, 1.0);

    let mut frame: drum_machine::AudioOut = machine.next_frame(SAMPLE_HZ);
    while frame[0] != 0.0 {
        println!("{:?}", frame);
        frame = machine.next_frame(SAMPLE_HZ);
    }

    machine.trigger(1, 1.0);
    machine.pause();
    println!("{:?}", machine.next_frame(SAMPLE_HZ));
    machine.unpause();
    println!("{:?}", machine.next_frame(SAMPLE_HZ));
    machine.stop();
    println!("{:?}", machine.next_frame(SAMPLE_HZ));
}
