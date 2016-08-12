#![feature(plugin)]
#![plugin(clippy)]


extern crate dsp;
mod drum_machine;

/// Not doing anything yet, just getting started
fn main() {
    let mut machine = drum_machine::Machine::new();
    println!("{:?}", machine.next());
    println!("{:?}", machine.volume);
}
