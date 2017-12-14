use std::collections::HashMap;
extern crate supertroupers;
extern crate hyphenation;
use supertroupers::gen::Markov;
use hyphenation::{Standard, FullTextHyphenation};
use hyphenation::Language::{English_US};
use hyphenation::hyphenator::Hyphenation;

// Load hyphenation data for American English from the pattern repository.


fn main() {
    let english_us = hyphenation::load(English_US).unwrap();
    let mut map = Markov::new();
    println!("initialized HashMap!");
    map.parse("everything is in its right place");
    println!("input parsed!");
    map.parse("each child a pram");
    map.parse("and bug a carapace");
    println!("input parsed!");
    println!("generator output:");
    let x = map.generate_sentence();
    let y: Standard  = x.fulltext_hyphenate(&english_us);
    let v: Vec<&str> = y.collect();
    println!("{:?}", v);
}
