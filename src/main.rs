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

use supertroupers::gen::Markov;
use supertroupers::util;
use supertroupers::http;
use supertroupers::http::LinesFeeder;
use std::sync::{Arc, Mutex};
use dialoguer::Confirmation;
use console::Term;

fn confirm() -> io::Result<(bool)> {
    Ok((Confirmation::new("Draw from your experience and create something new?").interact()?))

}

fn main() {
    let mut feed: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut feeder: LinesFeeder = LinesFeeder {
        queue: feed,
    };
    let term = Term::stdout();
    loop {

        http::BasicSearch::author_title(feeder.clone());
        if Confirmation::new("keep searching?").interact_on(&term).unwrap() {
            println!("Sweet, lets do it!");
        } else {
            println!("I didn't want to make a stupid poem anyways...");
            break
        }
    }
}
