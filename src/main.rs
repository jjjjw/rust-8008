#![feature(plugin)]
#![plugin(clippy)]


extern crate dsp;
mod drum_machine;

/// Not doing anything yet, just getting started
fn main() {
    let mut machine = drum_machine::Machine::new();
    machine.trigger(1, 1.0);
    println!("{:?}", machine.next());
    println!("{:?}", machine.next());
    println!("{:?}", machine.next());
    println!("{:?}", machine.next());
    println!("{:?}", machine.next());
    println!("{:?}", machine.next());
    println!("{:?}", machine.next());
    println!("{:?}", machine.volume);
}
