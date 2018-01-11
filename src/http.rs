//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
use reqwest;

use serde_json::{self, Value};
use util;

use std::sync::{Arc, Mutex};
use std::ops::DerefMut;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoemLines {
    lines: Vec<String>,
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
impl PoemLines {
    pub fn new()->PoemLines {
        PoemLines {
            lines: Vec::new()
        }
    }
    pub fn from_value(mut self, json_val: &Value) -> Result<(PoemLines),(serde_json::Error)> {
        let content: Vec<String> = serde_json::from_value(json_val.clone())?;
        self = PoemLines {
            lines: content,
        };
        return Ok((self));
    }
}
pub fn search_author_title(feeder: LinesFeeder) -> Result<(), reqwest::Error> {

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


pub fn extract_lines(req: Request, feeder: LinesFeeder) -> Result<Value, reqwest::Error> {

    let mut response = reqwest::get(&req.url)?;

    let json: Value = response.json()?;

    lines_search(json.clone(), feeder).expect("Something went wrong searching for lines!");

    return Ok((json));
}

pub fn lines_search(json_val: Value, mut feeder: LinesFeeder) -> Result<Value,serde_json::Error> {
    // let json_val: serde_json::Value = resp.json()?;
    match &json_val {
        &Value::Array(ref arr) => {
            println!("got Array!");
            let array_string: String = serde_json::to_string_pretty(&arr)?;
            println!("---------------------------------------------------------");
            println!("JSON Array: \n{}", &array_string);
            println!("---------------------------------------------------------");
            for obj_val in &arr[..] {
                match obj_val.get("lines") {
                    Some(content) => {
                        println!("found some lines in the Array!");
                        let poem = PoemLines::new().from_value(&content)?;
                        println!("---------------------------------------------------------");
                        println!("got line values! \n{}", 
                            &poem.lines.join("\n"));
                        println!("---------------------------------------------------------");
                        feeder.add_lines(poem.lines)
                            .expect("something went wrong adding lines from array!");
                    }
                    None => { 
                        println!("couldn't get any lines from this array.");
                        break;
                    }
                }
            }
        }
        &Value::Object(ref obj) => {
            println!("got Object!");
            let object_string: String = serde_json::to_string_pretty(&obj)?;
            println!("---------------------------------------------------------");
            println!("JSON Object: \n{}", &object_string);
            println!("---------------------------------------------------------");
            match &obj.get("lines") {
                &Some(content) => {
                    println!("found some lines in the Object!");
                    let poem = PoemLines::new().from_value(&content)?;
                    println!("---------------------------------------------------------");
                    println!("got line values! \n{}", &poem.lines.join("\n"));    
                    println!("---------------------------------------------------------");                
                    feeder.add_lines(poem.lines)
                        .expect("something went wrong adding lines from object!");
                }
                &None => println!("couldn't get any lines from this object!"),
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
        println!("---------------------------------------------------------");
        println!("    total lines stored:   {}", queued.len());
        println!("---------------------------------------------------------");
        return Ok((self.clone()));
    }
}
