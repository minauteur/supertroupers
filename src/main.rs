#[macro_use]
extern crate serde_derive;
extern crate supertroupers;
extern crate piston;
extern crate conrod;
extern crate hyphenation;
extern crate markov;
// extern crate term;
extern crate serde;
extern crate serde_json;
use supertroupers::poems;
use supertroupers::util;
use supertroupers::http::*;
use supertroupers::http;
use supertroupers::flavor;
use supertroupers::gen;
use std::sync::{Arc, Mutex};
extern crate colored;
use colored::*;
use markov::Chain;

#[feature = "term_size"]
#[feature = "hyphenation"]
extern crate textwrap; 
// #![feature(term_size, hyphenation)]



fn main() {
    let feed_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    // let chain = Chain::new();
    let feeder: http::LineSeed = http::LineSeed { queue: feed_store };
    flavor::welcome();
    let mut chain = Chain::new();
    loop {

        let request = http::Search::new().auth_title_inc();
        let response = handle(request).expect("Something went wrong handling request!");
        let data = match_value(response, &mut chain, feeder.clone()).expect(
            "Something went wrong searching for lines!",
        );
        let len = util::get_len(feeder.clone());
        util::poem_prompt(data, len);

    }
}
