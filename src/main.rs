#[macro_use]
extern crate serde_derive;

extern crate supertroupers;
// extern crate hyphenation;
extern crate markov;
// extern crate term;
extern crate serde;
extern crate serde_json;
use supertroupers::poems;
use supertroupers::util;
use supertroupers::http::*;
use supertroupers::http;
use std::sync::{Arc, Mutex};
extern crate colored;
use colored::*;

fn main() {
    let feed_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let feeder: http::LineSeed = http::LineSeed { queue: feed_store };
    println!(
        "\n{}",
        "                           WELCOME TO".bright_yellow()
    );
    println!("                       |----------------------|");
    println!(
        "                       |        {}{}{}{}{}         |",
        "S".white(),
        "U".yellow(),
        "P".bright_yellow(),
        "E".red(),
        "R".bright_red()
    );
    println!(
        "                       |      {}{}{}{}{}{}{}{}        |",
        "T".cyan(),
        "R".bright_purple(),
        "O".bright_green(),
        "U".green(),
        "P".purple(),
        "E".bright_purple(),
        "R".cyan(),
        "S".bright_green()
    );
    println!("                       |----------------------|");
    loop {

        let request = http::Search::new().auth_title_inc();
        let response = handle(request).expect("Something went wrong handling request!");
        let data = match_value(response, feeder.clone()).expect(
            "Something went wrong searching for lines!",
        );
        util::poem_prompt(feeder.clone());

    }
}
