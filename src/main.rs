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
use std::ops::Deref;
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
            println!("Sweet, lets do it!");
            let lines = match feeder.queue.lock() {
                Ok(vec)=>vec,
                Err(e) => e.into_inner(),
            };
            let deref = lines.deref().clone();
            if !deref.is_empty() {
                let mut chain: Chain<String> = Chain::new();
                chain.feed(deref);
                let output = chain.generate();
                let formatted = format!("Your Poem:\n{}", lines.deref().clone().join("\n"));
                println!("You swill your thoughts and words begin to swirl--an ORIGINAL thought takes shape!\n{}", formatted);                
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
    loop {

        http::BasicSearch::author_title(feeder.clone());
        poem_prompt(feeder.clone());
    }
}
