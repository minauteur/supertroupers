//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
use reqwest;

use serde_json::{self, Value};
use util;
use poems::*;
use std::sync::{Arc, Mutex};
use std::ops::DerefMut;
use colored::*;

use std::io::Error;
//use std::ops::Try;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    pub authors: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct LinesFeeder {
    pub queue: Arc<Mutex<Vec<String>>>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct PoemLines {
//     lines: Vec<String>,
// }


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
// pub fn build_search_prompt() -> Result<(Value), reqwest::Error> {


//     let title = util::read_in_ln();

//     println!("checking author value... author == {:?}", author);
//     println!("checking title value... title == {:?}", title);

//     let request: Request = Request::new().with_params(author, title);

//     ;


//     return Ok((response));

//     // let poem = get_lines(serialized);
//     // return Ok(());
// }



impl Search {
    pub fn new() -> Search {
        let request = String::from("http://poetrydb.org/");
        Search {
            url: request,
            options: ReqOpts::new(),
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
            println!("No author or title given... \nreturning list of authors...");
            let list_authors = format!("author");
            &self.url.push_str(&list_authors);
        } else if self.options.author.is_some() && self.options.title.is_some() {
            println!("searching for substring matches by author name and work title given...");
            let author_and_title = format!("author,title/{};{}", &a.trim_right(), &t.trim_right());
            &self.url.push_str(&author_and_title);
        } else if self.options.author.is_none() && self.options.title.is_some() {
            let single_title = format!("title/{}", &t.trim_right());
            &self.url.push_str(&single_title);
        }
        println!("request string: {}", &self.url);
        Search {
            url: self.url.clone(),
            options: self.options.clone(),
        }
    }
}


pub fn handle(search: Search) -> Result<Value, reqwest::Error> {

    // let mut response = reqwest::get(&req.url)?;
    let mut response = reqwest::get(&search.url)?;

    let json: Value = response.json()?;

    return Ok((json));
}

pub fn match_value(json_val: Value, mut feeder: LinesFeeder) -> Result<Value, serde_json::Error> {
    // let json_val: serde_json::Value = resp.json()?;
    match &json_val {
        &Value::Array(ref arr) => {
            println!("got an Array of Poems!");
            for obj_val in &arr[..] {
                if let Ok(p) = Poem::new().from_value(&obj_val) {
                    println!("Got a Poem!");
                    feeder.add_lines(p.lines.clone()).expect(
                        "couldn't get lines from Poem struct!",
                    );
                    println!(
                        "\nPoem: \"{}\",\nAuthor: {},\n  Lines: {}",
                        p.title,
                        p.author,
                        p.line_count
                    );
                }
            }
        }
        &Value::Object(ref obj) => {
            println!("got Object!");
            if let Ok(p) = Poem::new().from_value(&json_val) {
                println!("Got a Poem!");
                feeder.add_lines(p.lines.clone()).expect(
                    "couldn't get lines from Poem!",
                );
                println!(
                    "\nPoem: \"{}\",\nAuthor: {},\n  Lines: {}",
                    p.title,
                    p.author,
                    p.line_count
                );
            }
        } 
        _ => {
            println!("got... something else!");
            println!("Didn't know enough to serialize this!");
        }
    }
    return Ok((json_val));
}

impl LinesFeeder {
    pub fn new() -> LinesFeeder {
        let arc_mut_vec: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        return LinesFeeder { queue: arc_mut_vec };
    }

    pub fn add_lines(&mut self, content: Vec<String>) -> Result<LinesFeeder, Error> {

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
