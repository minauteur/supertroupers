//!Utilities Module
//!Various Helper functions and error definitions used throughout the project live here
#[cfg(feature = "term_size")]
#[cfg(feature = "hyphenation")]
use supertroupers::gen;
use http::LineSeed;
use poems::{AuthorsList,WorksList};
use serde_json;
use colored::*;
use markov::Chain;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io;
use std::io::Write;

use std::error::Error;
use std::fs::File;

static AUTHOR: &'static str = "names.json";
static TITLE: &'static str = "title.json";

pub fn write_poem_to_file(poem: Vec<String>, author: String, title: String) {

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("poems.txt")
        .unwrap();
    // let mut writer = BufWriter::new(&mut file);
    if let Err(e) = writeln!(file, "\r\n") {
        println!("{}", e);
    }
    if let Err(e) = writeln!(file, "  \"{}\"\r\n\r\n", title) {
        println!("{}", e);
    }
    for line in poem {
        if let Err(e) = writeln!(file, "    {}\r\n", line) {
            println!("{}", e);
        }
    }
    if let Err(e) = writeln!(file, "\r\n    --{}\r\n", author) {
        println!("{}", e);
    }
    if let Err(e) = writeln!(
        file,
        "\r\n------------------------------------------------------\r\n"
    )
    {
        println!("{}", e);
    }
    println!(
        "{} See poems.txt in your supertroupers folder to view output.",
        " Success!".green()
    );
}


pub fn read_authors_from_file() -> Result<AuthorsList, Box<Error>> {
    // Open the file in read-only mode.
    let path = PathBuf::from(AUTHOR);
    let file = File::open(&path)?;

    // Read the JSON contents of the file as an instance of `AuthorsList`.
    let list: AuthorsList = serde_json::from_reader(file)?;

    // Return the `List`.
    Ok(list)
}

pub fn read_titles_from_file() -> Result<WorksList, Box<Error>> {
    // Open the file in read-only mode.
    let path = PathBuf::from(TITLE);
    let file = File::open(&path)?;

    // Read the JSON contents of the file as an instance of `AuthorsList`.
    let list: WorksList = serde_json::from_reader(file)?;

    // Return the `List`.
    Ok(list)
}

pub fn read_y_n() -> bool {
    loop {
        println!(
            "         {} {} or {} {}",
            "??".clear(),
            "Y".bright_green(),
            "N".bright_red(),
            "??".clear()
        );
        let i = read_in_ln();
        match i {
            Some(s) => {
                match s.trim() {
                    "Y" | "y" => {
                        //println!("read \"Y\" or \"y\". Confirmed!");
                        return true;
                    }
                    "N" | "n" => {
                        //println!("read \"N\" or \"n\". Canceled!");
                        return false;
                    }
                    _ => {
                        println!("Try again... please, enter a single character: Y, y, N, or n");
                        continue;
                    }
                }
            }
            None => {
                println!("lets try that again, shall we?");
                continue;
            }
        }
    }
}

pub fn which_prompt(cond_1:&String, cond_2: &String) -> bool {
    loop {
        println!(
            "         {} {} or {} {}",
            "??".clear(),
            cond_1.bright_green(),
            cond_2.bright_red(),
            "??".clear()
        );
        let i = read_in_ln();
        match i {
            Some(s) => {
                    if s.trim() == cond_1.to_string() {
                        //println!("read \"Y\" or \"y\". Confirmed!");
                        return true;
                    } else if s.trim() == cond_2.to_string() {
                        return false;
                    } else {
                    
                        println!("Try again... please, enter either \"{}\", or \"{}\"", cond_1, cond_2);
                        continue;
                    }
                }
            None => {
                println!("lets try that again, shall we?");
                continue;
            }
        }
    }
}

pub fn read_in_ln() -> Option<String> {
    let mut out = String::new();
    match io::stdin().read_line(&mut out) {
        Ok(..) => {
            if !out.trim().is_empty() {
                if out == "\n".to_string() {
                    println!("Nothing entered.");
                    return None;
                } else if out == "\r".to_string() {
                    println!("Nothing entered.");
                    return None;
                } else {
                    return Some(out);
                }
            } else {
                return None;
            }
        }
        Err(error) => {
            println!("error reading input: {}", error);
            return None;
        }
    }
}

pub fn read_int() -> i32 {
    loop {
        println!("Enter a whole number...");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(..) => {
                match user_input.trim().parse::<i32>() {
                    Ok(number) => return number,
                    Err(..) => {
                        println!(
                            "that wasn't a number!\nYou entered: \"{}\"",
                            user_input.trim()
                        );
                        continue;
                    }
                }
            }
            Err(error) => {
                println!("error reading input: {}", error);
                println!("let's try again");
                continue;
            }
        };
    }
}
use super::gen;

pub fn poem_prompt(chain: &mut Chain<String>, lines_read: usize) -> &Chain<String> {
    println!("Do you want to pause and curate a poem?");
    if read_y_n() {
        println!("Sweet! let\'s find an author!\n");
        //in order to access our persistent storage, within its structure Arc<Mutex<Vec<String>
        //we first need to acquire a "lock", which confirms to us no other thread can attempt
        //to access the underlying data. The error returns a "view" of the data anyway as a
        //fail-safe and because we only need to read what's in the store, that's fine.
        // let lock = match feeder.queue.lock() {
        //     Ok(vec) => vec,
        //     Err(e) => e.into_inner(),
        // };
        //Now that we've acquired our MutexGuard from matching against .lock(),
        //we have an Arc<Vec<String>
        //it helps to think about the data you need as trapped in a room puzzle.
        //It needs to move from the inside-out! Mutex-->MutexGuard-->Arc-->Deref-->Usable Data
        //This level of integrity is most definitely overkill right now, but it will help us
        //immensely if we end up including any online/multiplayer features requiring server-related
        //functionality. Deref is a trait implemented for the Arc type, included in rust's "ops"
        //package, that converts an Arc<T> into T while maintaining an active count in memory of
        //accessors of T (via the deref trait), ARC meaning Atomic Reference Counter
        // let vec = lock.deref().clone();
        //after dereferencing our Arc, all we have left to do is...
        //give the Vec<String> to our seed/generate function.
        //We need to clone the dereferenced Arc<String<Vec>> to avoid it getting wrapped
        //again in the next function. this prevents it
        //from being a reference, which gen::seed_and_generate() won't accept
        //but first--we need to make sure it isn't empty!
        if !&chain.is_empty() {
            gen::seed_and_generate(&chain, lines_read);
            return chain;
        } else {
            println!("Yeah, you should probably read more...");
        }
    } else {
        println!("I didn't want to make a stupid poem anyways...");
    }
    return chain;
}
pub fn get_len(feeder:LineSeed) -> usize {
    let lock = match feeder.queue.lock() {
        Ok(vec)=> vec,
        Err(e)=> e.into_inner(),
    };
    return lock.len()
}
