#[macro_use]
extern crate serde_derive;

extern crate supertroupers;
extern crate hyphenation;
extern crate markov;
extern crate dialoguer;
extern crate console;

extern crate serde;
extern crate serde_json;
use std::io;

#[macro_use]
extern crate text_io;
use markov::Chain;
use std::ops::{Deref, DerefMut};
use supertroupers::gen::Markov;
use supertroupers::util;
use supertroupers::http;
use supertroupers::http::LinesFeeder;
use std::sync::{Arc, Mutex};
use dialoguer::Confirmation;
use console::Term;

fn poem_prompt(feeder: LinesFeeder) {
        println!("Do you want to pause and write a poem?");    
        if util::read_y_n() {
            let mut chain = Chain::new();
            println!("Sweet, lets do it!");
            let mut lock = match feeder.queue.lock() {
                Ok(vec)=>vec,
                Err(e) => e.into_inner(),
            };
            let deref = lock.deref();
            if !&deref.is_empty() {
                for string in deref.clone() {
                    chain.feed_str(&string.clone());
                }
                // chain.feed(deref);
                // chain.feed(line_words);
                if deref.len() > 50 {
                    println!("We've stored quite a few lines. \n Would you like to cut it down to 20 lines?")
                } if util::read_y_n() {
                    for line in chain.str_iter_for(20) {
                        println!("{}", chain.generate_str());
                    }
                } else {
                    for line in chain.str_iter_for(deref.len()) {
                        println!("{}", chain.generate_str());
                    }
                }
                // let output = chain.generate();
                // let formatted = format!("Your Poem:\n{:?}", chain.generate_str_from_token(&token));
              
            } else {
                    println!("Yeah, you should probably read more...");
            }
        } else {
            println!("I didn't want to make a stupid poem anyways...");       
        }

}


fn main() {
    let mut feed: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut feeder: LinesFeeder = LinesFeeder {
        queue: feed,
    };
    let mut chain: Chain<String> = Chain::new();
    loop {

        http::BasicSearch::author_title(feeder.clone());
        poem_prompt(feeder.clone());
    }
}
