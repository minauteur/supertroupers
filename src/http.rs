//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
use reqwest;
use serde_json::{self, Value};
use markov::Chain;

use util;
use poems::*;
use colored::*;

use std::io::Error;
use std::sync::{Arc, Mutex};
use std::ops::DerefMut;

#[derive(Debug, Clone)]
pub struct LineSeed {
    pub queue: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    url: String,
    options: ReqOpts,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqOpts {
    author: Option<String>,
    title: Option<String>,
}
impl ReqOpts {
    pub fn new() -> ReqOpts {
        ReqOpts {
            author: None,
            title: None,
        }
    }
}
impl Search {
    pub fn new() -> Search {
        Search {
            url: String::from("http://poetrydb.org/"),
            options: ReqOpts::new(),
        }
    }
    pub fn auth_title_inc(&mut self) -> Search {
        self.author_prompt();
        let a = match self.options.author.clone() {
            Some(auth_name) => auth_name,
            None => String::from(""),
        };
        self.title_prompt();
        let t = match self.options.title.clone() {
            Some(text_name) => text_name,
            None => String::from(""),
        };
        if self.options.author.is_some() && self.options.title.is_none() {
            println!(
                "No title given... \n
                returning authors with names matching substrings from provided input..."
            );
            let single_author = format!("author/{}/title", &a.trim_right());
            &self.url.push_str(&single_author);
        } else if self.options.author.is_none() && self.options.title.is_none() {
            println!("No author or title given... \nWould you like to return a list of authors or titles?\n");
            if util::which_prompt(&format!("author"), &format!("title")) {
                self.url = String::from("http://poetrydb.org/author");
                let list = AuthorsList::new();
                println!("authors:\n{}\n", list.authors.join("\n"));
            } else {
                self.url = String::from("http://poetrydb.org/title");
                let list = WorksList::new();
                println!("titles: \n{}\n", list.titles.join("\n"));
            }
            
        } else if self.options.author.is_some() && self.options.title.is_some() {
            println!("searching for substring matches by author name and work title given...");
            let author_and_title = format!("author,title/{};{}", &a.trim_right(), &t.trim_right());
            &self.url.push_str(&author_and_title);
        } else if self.options.author.is_none() && self.options.title.is_some() {
            let single_title = format!("title/{}", &t.trim_right());
            &self.url.push_str(&single_title);
        }
        println!("request string: {}\n", &self.url);
        Search {
            url: self.url.clone(),
            options: self.options.clone(),
        }
    }
    fn author_prompt(&mut self) -> Self {
        println!("{}{}", "Search for an ".clear(), "Author?".green());
        self.options.author = util::read_in_ln();
        return self.to_owned();
    }
    fn title_prompt(&mut self) -> Self {
        println!("{}{}", "and a ".clear(), "title?".green());
        self.options.title = util::read_in_ln();
        return self.to_owned();
    }
}


pub fn handle(search: Search) -> Result<Value, reqwest::Error> {
    let mut response = reqwest::get(&search.url)?;
    let json: Value = response.json()?;

    return Ok((json));
}

pub fn match_value(json_val: Value, mut chain: Chain<String>, mut feeder: LineSeed) -> Result<Chain<String>, serde_json::Error> {
    // let json_val: serde_json::Value = resp.json()?;
    // let feed_clone = feeder.clone();
    // let mut lock = match feed_clone.queue.lock() {
    //         Ok(vec) => vec,
    //         Err(e) => e.into_inner(),
    // };
    // let v = lock.deref_mut();
    match &json_val {
        &Value::Array(ref arr) => {
            println!("got Array!\n");
            for obj_val in &arr[..] {
                if let Ok(p) = Poem::new().from_value(&obj_val) {
                    // println!("Got a Poem!");
                    if p.line_count > 0 {
                        println!(
                            "\nTitle: \"{}\",\nAuthor: {},\nLines: {}",
                            p.title,
                            p.author,
                            p.line_count
                        );
                        for line in p.lines.clone() {
                            chain.feed_str(&line);
                        }
                        feeder.add_lines(p.lines.clone())
                            .expect("couldn't get lines!");
                    }
                } 
            }
        }
        &Value::Object(..) => {
            println!("got Object!");
            if let Ok(p) = Poem::new().from_value(&json_val) {
                // println!("Got a Poem!");
                if p.line_count > 0 {
                    println!(
                        "\nPoem: \"{}\",\nAuthor: {},\nLines: {}",
                        p.title,
                        p.author,
                        p.line_count
                    );
                    for line in p.lines.clone(){
                        chain.feed_str(&line);
                    }
                    feeder.add_lines(p.lines.clone())
                        .expect("couldn't get lines!");
                }

            }
        } 
        _ => {
            println!("got... something else!");
            println!("Didn't know enough to serialize this!");
        }
    }
    // let mut lock = match feeder.queue.lock() {
    //         Ok(vec) => vec,
    //         Err(e) => e.into_inner(),
    // };
    // let vec = lock.deref_mut();
    // for line in vec {
    //     chain.feed_str(&line);
    // }
    return Ok((chain));
}

impl LineSeed {
    pub fn new() -> LineSeed {
        let arc_mut_vec: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        return LineSeed { queue: arc_mut_vec };
    }

    pub fn add_lines(&mut self, content: Vec<String>) -> Result<LineSeed, Error> {

        let mut queued = match self.queue.lock() {
            Ok(vec) => vec,
            Err(e) => e.into_inner(),
        };
        for each_line in content {
            if !each_line.is_empty() {
                each_line.trim();
                queued.deref_mut().push(each_line.to_owned());
            }
            // if we need individual words
            // for word in line.split_whitespace() {
            //     queued.deref_mut().push(word.clone().to_owned());
            // }
        }
        println!(
            "{}",
            "---------------------------------------------------------".green()
        );
        println!("    total lines stored:   {}", queued.len());
        println!(
            "{}",
            "---------------------------------------------------------".green()
        );
        return Ok((self.clone()));
    }
}
