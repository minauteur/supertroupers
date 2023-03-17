extern crate supertroupers;
extern crate tokio;
// extern crate piston;
// extern crate conrod;
extern crate hyphenation;
extern crate markov;
extern crate serde;
extern crate serde_json;
use supertroupers::util;
use supertroupers::http::*;
use supertroupers::http;
use supertroupers::flavor;
use std::sync::{Arc, Mutex};
use markov::Chain;




#[tokio::main]
async fn main() {
    let feed_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    // let chain = Chain::new();
    let feeder: http::LineSeed = http::LineSeed { queue: feed_store };
    flavor::welcome();
    let mut chain = Chain::new();
    loop {

        let request = http::Search::new().auth_title_inc();
        let response = handle(request).await.expect("");
        let data = match_value(response, &mut chain, feeder.clone()).expect(
            "Something went wrong searching for lines!",
        );
        let len = util::get_len(feeder.clone());
        // println!("length: {}\ndata: {:#?}", len, data);
        util::poem_prompt(data, len);

    }
}
