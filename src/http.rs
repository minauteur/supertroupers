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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProblem {
    pub status: usize,
    pub reason: String,
}

impl SearchProblem {
    pub fn from_value(json: &Value) -> bool {
        if let Ok(serialized) = serde_json::from_value(json.clone()) {
            let serialized: SearchProblem = serialized;
            println!(
                "Search Problem! \nstatus: {:?}\n reason: {:?}",
                serialized.status,
                serialized.reason
            );
            return true;
        } else {
            return false;
        }
    }
}

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
            println!(
                "No author or title given... \nWould you like to return a list of authors or titles?\n"
            );
            if util::which_prompt(&format!("author"), &format!("title")) {
                self.url = String::from("http://poetrydb.org/author");
                let list = AuthorsList::new();
                println!("\n  {}\n", list.authors.join("\n  ").bright_purple());
            } else {
                self.url = String::from("http://poetrydb.org/title");
                let list = WorksList::new();
                let mut list_str = list.titles.join("\"\n  \"");
                list_str.push_str("\"");
                println!("\n  {}\n", list_str.bright_green());
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

// pub fn check_non_poem(json: &Value)->Result<String, serde_json::Error> {

// }

pub fn handle(search: Search) -> Result<Value, reqwest::Error> {
    println!(
        "request sent!, \nA NOTE: {}",
        "generic searches (eg \"a\" for author and \"s\" for title) may take longer to process!"
            .bright_yellow()
    );
    let mut response = reqwest::get(&search.url)?;
    println!("response received!");
    let json: Value = response.json()?;
    println!("got json from response! Matching value for usable data...");
    return Ok((json));
}

pub fn match_value(
    json_val: Value,
    chain: &mut Chain<String>,
    mut feeder: LineSeed,
) -> Result<&mut Chain<String>, serde_json::Error> {

    match &json_val {
        &Value::Array(ref arr) => {
            println!("got Array!\n");
            for obj_val in &arr[..] {

                if let Ok(p) = Poem::new().from_value(&obj_val) {
                    // println!("Got a Poem!");
                    if p.line_count > 0 {
                        println!(
                            "\nTitle: \"{}\",\nAuthor: {},\nLine Count: {}\nLines: \n  {}",
                            p.title,
                            p.author,
                            p.line_count,
                            p.lines.join("\n  ")
                        );
                        for line in &p.lines {
                            chain.feed_str(&line);
                        }
                        feeder.add_lines(p.lines).expect("couldn't get lines!");
                    }

                } else {
                    println!("Not a poem. here's what we got instead: \n{}", serde_json::to_string_pretty(&obj_val)?.blue());
                }
                // }
            }

            return Ok(chain);
        }
        &Value::Object(..) => {
            println!("got Object!");

            if let Ok(p) = Poem::new().from_value(&json_val) {
                // println!("Got a Poem!");
                if p.line_count > 0 {
                    println!(
                        "\nTitle: \"{}\",\nAuthor: {},\nLine Count: {}\nLines: \n  {}",
                        p.title,
                        p.author,
                        p.line_count,
                        p.lines.join("\n  ")
                    );
                    for line in &p.lines {
                        chain.feed_str(&line);
                    }
                    feeder.add_lines(p.lines).expect("couldn't get lines!");
                }

            } else {
                    println!("Not a poem. here's what we got instead: \n{}", serde_json::to_string_pretty(&json_val)?.bright_blue());
                }
            return Ok(chain);
        } 
        _ => {
            println!("got... something else!");
            println!("Didn't know enough to serialize this!");
            println!(
                "Here's what JSON we got: \n{}",
                serde_json::to_string_pretty(&json_val)?.bright_yellow()
            );

        }
    }
    return Ok((chain));
    // }
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
