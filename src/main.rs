extern crate supertroupers;
// extern crate piston;
// extern crate conrod;
#[macro_use]
extern crate serde_json;
extern crate hyphenation;
extern crate markov;
extern crate serde;
use supertroupers::util;
use supertroupers::http::*;
use supertroupers::http;
use supertroupers::flavor;
use std::sync::{Arc, Mutex};
use markov::Chain;





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
