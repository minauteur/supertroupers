//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
use reqwest;

use serde_json;
use util;

use std::sync::{Arc, Mutex};
use std::ops::DerefMut;

use std::io::Error;
//use std::ops::Try;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    authors: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct LinesFeeder {
  pub queue: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poem {
    author: String,
    linecount: i32,
    lines: Vec<String>,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBuilder {
    author: Option<String>,
    title: Option<String>,
}

impl RequestBuilder {

}


pub fn search_author_title(feeder: LinesFeeder) -> Result<(),reqwest::Error> {

        println!("Search for an Author?");
        let author = util::read_in_ln();

        println!("and a title?");
        let title = util::read_in_ln();

        println!("checking author value... author == {:?}", author);
        println!("checking title value... title == {:?}", title);

        let request: Request = Request::new().with_params(author, title);
        extract_lines(request, feeder)?;
        // let poem = get_lines(serialized);
        return Ok(());
}



impl Request {
    pub fn new() -> Request {
        let request = String::from("http://poetrydb.org/");
        Request { url: request }
    }

    pub fn with_params(&mut self, author: Option<String>, title: Option<String>) -> Request {

        let a = match author.clone() {
            Some(auth_name) => auth_name,
            None => String::from(""),
        };
        let t = match title.clone() {
            Some(text_name) => text_name,
            None => String::from(""),
        };
        if author.is_some() && title.is_none() {
            println!(
                "No title given... \n
                returning authors with names matching substrings from provided input..."
            );
            let single_author = format!("author/{}/title", &a.trim_right());
            &self.url.push_str(&single_author);
        } else if author.is_none() && title.is_none() {
            println!("No author or title given... \nreturning list of authors...");
            let list_authors = format!("author");
            &self.url.push_str(&list_authors);
        } else if author.is_some() && title.is_some() {
            println!("searching for substring matches by author name and work title given...");
            let author_and_title = format!("author,title/{};{}", &a.trim_right(), &t.trim_right());
            &self.url.push_str(&author_and_title);
        } else if author.is_none() && title.is_some() {
            let single_title = format!("title/{}", &t.trim_right());
            &self.url.push_str(&single_title);
        }
        println!("request string: {}", &self.url);
        Request { url: self.url.clone() }
    }
}

pub fn extract_lines(req: Request, feeder: LinesFeeder) -> Result<serde_json::Value, reqwest::Error> {
    
    let mut response = reqwest::get(&req.url)?;
    
    let json: serde_json::Value = response.json()?;
    
    lines_search(json.clone(), feeder);
    
    return Ok((json));
}

pub fn lines_search(mut json_val: serde_json::Value, mut feeder: LinesFeeder) -> Result<serde_json::Value, serde_json::Error> {
        // let json_val: serde_json::Value = resp.json()?; 
        match &json_val {
            &serde_json::Value::Array(ref arr) => {
                println!("got Array!");
                for obj_val in &arr[..] {
                    match obj_val.get("lines") {
                        Some(content) => {
                            println!("got some lines out of the array! {}", &content.to_string());
                            let mut lines_found: Vec<String> = Vec::new();
                            for line in serde_json::to_string_pretty(content)?.lines() {
                                if !line.is_empty() {
                                    line.trim();
                                    lines_found.push(line.to_string());
                                }
                            }
                            feeder.add_lines(lines_found);
                        }
                        None => println!("couldn't get any lines from this array."),
                    }
                }
                let array_string: String = serde_json::to_string_pretty(&arr)?;
                println!("Array: {}", &array_string);
            }
            &serde_json::Value::Object(ref obj) => {
                println!("got Object!");
                match &obj.get("lines") {
                    &Some(content) => {
                        println!("got some lines from the object! {}", &content.to_string());
                        let mut lines_found: Vec<String> = Vec::new();
                        for line in serde_json::to_string_pretty(content)?.lines() {
                            if !line.is_empty() {
                                line.trim();
                                lines_found.push(line.to_string());
                            }
                        }
                        feeder.add_lines(lines_found);
                    }
                    &None => println!("couldn't get any lines from this object!"),
                }
                let object_string: String = serde_json::to_string_pretty(&obj)?;
                println!("Object Searched for lines: \n{}", &object_string);
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
        return LinesFeeder {
            queue: arc_mut_vec,
        }
    }

    pub fn add_lines(&mut self, lines: Vec<String>) -> Result<LinesFeeder, Error> {

        let mut queued = match self.queue.lock() {
            Ok(vec)=>vec,
            Err(e) => e.into_inner(),
        };
        for each_line in lines.iter() {
            queued.deref_mut().push(each_line.to_owned());
            // if we need individual words
            // for word in line.split_whitespace() {
            //     queued.deref_mut().push(word.clone().to_owned());
            // }
        }
        println!("total lines stored: {}", queued.len());
        return Ok((self.clone()));
    }
}
