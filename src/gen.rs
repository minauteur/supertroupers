//!Gen Module
//!This file contains behaviors and functions critical to text generation
extern crate rand;
use self::rand::{thread_rng, Rng};
use std::collections::HashMap;
use markov::Chain;
use hyphenation::{Standard, FullTextHyphenation};
use hyphenation::Language::English_US;
use hyphenation::hyphenator::Hyphenation;
use util;

//Example invocation moved from main.rs to avoid clutter
//    // let english_us = hyphenation::load(English_US).unwrap();
// let mut map = Markov::new();
// println!("What is your name, bard!?");
// let name: String = read!("{}");
// println!("Hail, {}, mucho gusto!", &name);
// println!("give us a line!");
// let term_seed: String = read!("{}");
// map.parse(&term_seed);
// println!("and another?");
// let term_two: String = read!("{}");
// map.parse(&term_two);

// println!("generator output:");
// let x = map.generate_sentence();
// println!("raw OP: {}", &x);
// let y: Standard = x.fulltext_hyphenate(&english_us);
// let v: Vec<&str> = y.collect();
// println!("{:?}", v);
// //util::read_file();


pub struct Markov {
    map: HashMap<String, Vec<String>>,
}

impl Markov {
    //create a new, empty map
    pub fn new() -> Markov {
        Markov { map: HashMap::new() }
    }
    //sentence parsing helper
    pub fn parse(&mut self, sentence: &str) {
        let words = sentence.split(" ").collect::<Vec<&str>>();
        let word_count = words.len();

        for n in 0..word_count {
            if n + 2 < word_count {
                let key = format!("{} {}", words[n], words[n + 1]);
                let value = words[n + 2];
                //have to define insert since we are working with our own custom type!
                self.insert(key, value.to_string())
            }
        }
    }
    //insert definition for custom type.
    fn insert(&mut self, key: String, value: String) {
        if self.map.contains_key(&key) {
            let current_value = self.map.get_mut(&key).unwrap();
            current_value.push(value);
        } else {
            self.map.insert(key, vec![value]);
        }
    }
    //generate a sentence
    pub fn generate_sentence(self) -> String {
        let mut rng = thread_rng();
        let keys = self.map.keys().collect::<Vec<&String>>();

        let mut key = rng.choose(&keys)
            .expect("could not get random value")
            .to_string();
        let mut sentence = key.clone();

        loop {
            match self.map.get(&key) {
                Some(values) => {
                    let value = rng.choose(values).expect("could not get value");
                    sentence = format!("{} {}", sentence, value);

                    key = next_key(&key, value);
                }
                None => break,
            }
        }
        sentence
    }
}
//traversal between keys and stops
fn next_key(key: &str, value: &str) -> String {
    let last_word = key.split(" ").last().expect("could not get last word");
    format!("{} {}", last_word, value)
}

pub fn seed_and_generate(seed_store: Vec<String>) {
    let mut chain = Chain::new();
    for string in &seed_store {
        chain.feed_str(string);
    }
    if seed_store.len() > 30 {
        println!("More than 30 lines seeded. Specify a number of lines to generate?");
        println!("(Entering N/n will generate lines == number of seeds provided)");
    } if util::read_y_n() {
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
