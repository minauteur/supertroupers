use std::collections::HashMap;
extern crate supertroupers;
extern crate hyphenation;
extern crate markov;
extern crate serde;
#[macro_use] extern crate text_io;
use supertroupers::gen::Markov;
use supertroupers::util;

use hyphenation::{Standard, FullTextHyphenation};
use hyphenation::Language::{English_US};
use hyphenation::hyphenator::Hyphenation;

// Load hyphenation data for American English from the pattern repository.


fn main() {
    let english_us = hyphenation::load(English_US).unwrap();
    let mut map = Markov::new();
    println!("What is your name, bard!?");
    let name: String = read!("{}");
    println!("Hail, {}, mucho gusto!", &name);
    println!("give us a line!");
    let term_seed: String = read!("{}\n");
    //println!("term_seed {}", &term_seed);
    map.parse(&term_seed);
    println!("and another?");
    let term_two: String = read!("{}\n");
    map.parse(&term_two);
    println!("one more!");
    let term_three: String = read!("{}\n");
    map.parse(&term_three);
    // map.parse("each child a pram");
    //map.parse("and bug a carapace");
    //println!("input parsed!");
    println!("generator output:");
    let x = map.generate_sentence();
    println!("raw OP: {}", &x);
    let y: Standard  = x.fulltext_hyphenate(&english_us);
    let v: Vec<&str> = y.collect();
    println!("{:?}", v);
}
