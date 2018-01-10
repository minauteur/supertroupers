//!Gen Module
//!This file contains behaviors and functions critical to text generation
extern crate rand;

use markov::Chain;

use util;

pub fn seed_and_generate(seed_store: Vec<String>) {
    let mut chain = Chain::new();
    for string in seed_store.clone() {
        chain.feed_str(&string);
    }
    if seed_store.len() > 30 {
        println!("More than 30 lines seeded. Specify a number of lines to generate?");
        println!("N/n generates a number of lines equal to the number of lines read.");
        println!("Y/y prompts for a whole number, generating lines equal to that number.");
    
    if util::read_y_n() {
        let num = util::read_int();
        for line in chain.str_iter_for(num as usize) {
            if !line.is_empty() {
                println!("    {}", chain.generate_str());
            } else {
                println!("--------------------------------------------------------");
            }
        }
    }} else {
        for line in chain.str_iter_for(seed_store.len()) {
            if !line.is_empty() {
                println!("    {}", chain.generate_str());
            } else{
                println!("--------------------------------------------------------");
                
            }
        }
    }
}
