#[macro_use]
extern crate serde_derive;

extern crate supertroupers;
// extern crate hyphenation;
extern crate markov;
// extern crate term;
extern crate serde;
extern crate serde_json;

use supertroupers::util;
use supertroupers::http;
use std::sync::{Arc, Mutex};


fn main() {
    let feed_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let feeder: http::LinesFeeder = http::LinesFeeder { queue: feed_store };

    loop {

        http::search_author_title(feeder.clone());
        util::poem_prompt(feeder.clone());

    }
}
