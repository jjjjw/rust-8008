#![feature(plugin)]
#![plugin(clippy)]

extern crate instrument;
mod drum_machine;

/// Not doing anything yet, just getting started
fn main() {
    let machine = drum_machine::Machine::new();
    println!("{:?}", machine.volume);
}
