//!Gen Module
//!This file contains behaviors and functions critical to text generation
extern crate rand;

use markov::Chain;

use util;

pub fn seed_and_generate(seed_store: Vec<String>) {
    let mut chain = Chain::new();
    for string in &seed_store {
        chain.feed_str(string);
    }
    if seed_store.len() > 30 {
        println!("More than 30 lines seeded. Specify a number of lines to generate?");
        println!("(Entering N/n will generate lines == number of seeds provided)");
    }
    if util::read_y_n() {
        let num = util::read_int();
        for line in chain.str_iter_for(num as usize) {
            println!("{}", chain.generate_str());
        }
    } else {
        for line in chain.str_iter_for(seed_store.len()) {
            println!("{}", chain.generate_str());
        }
    }
}
