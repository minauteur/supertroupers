#[macro_use]
extern crate serde_derive;

extern crate supertroupers;
extern crate hyphenation;
extern crate markov;

extern crate serde;
extern crate serde_json;
use std::io;

#[macro_use]
extern crate text_io;

use supertroupers::gen::Markov;
use supertroupers::util;
use supertroupers::http;
use supertroupers::http::LinesFeeder;
use std::sync::{Arc, Mutex};

fn main() {
    let mut feed: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut line_container: LinesFeeder = LinesFeeder {
        queue: feed,
    };
    
    loop {

        http::BasicSearch::author_title();

    }
}
