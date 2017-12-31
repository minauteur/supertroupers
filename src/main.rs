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


fn main() {
    loop {
    
    http::BasicSearch::author_title();

    }
}
