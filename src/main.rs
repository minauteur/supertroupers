use std::collections::HashMap;
extern crate supertroupers;
use supertroupers::gen::Markov;

fn main() {
    let mut map = Markov::new();
    println!("initialized HashMap!");
    map.parse("I really like reeses but I do not really like almond joys.");
    println!("input parsed!");
    map.parse("I would really like to travel the world.");
    println!("input parsed!");
    println!("generator output:");
    println!("{:?}", map.generate_sentence());
}
