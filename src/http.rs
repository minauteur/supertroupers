//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
 #![feature(try_trait)]
use std::path::{Path, PathBuf};
use reqwest;
#[macro_use()]
use serde_derive;
use std::io;
use std::io::Read;

use serde_json;
use serde::{Serialize, Deserialize};
use serde;
use util;
use text_io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};
use std::result;
use std::sync;
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

pub struct AuthorWorks {
    author: String,
    works: Vec<SinglePoem>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poem {
    author: String,
    linecount: i32,
    lines: Vec<String>,
    title: String,
}


pub struct SinglePoem {
    title: String,
    author: String,
    lines: Vec<String>,
    l_num: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBuilder {
    author: Option<String>,
    title: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    url: String,
}

impl RequestBuilder {
    pub fn new() -> Request {
        let mut request = String::from("http://poetrydb.org/");
        Request { url: request }
    }
}
pub struct BasicSearch {
    results: String,
}



impl BasicSearch {
    pub fn author_title(mut feeder: LinesFeeder) -> BasicSearch {

        println!("Search for an Author?");
        let author = util::read_in_ln();

        println!("and a title?");
        let title = util::read_in_ln();

        println!("checking author value... author == {:?}", author);
        println!("checking title value... title == {:?}", title);

        let mut req: Request = RequestBuilder::new().with_params(author, title);
        let resp = get_response(req);
        let serialized = serialize(resp);
        // let poem = get_lines(serialized);
        let string = pretty_print(serialized, feeder);
        return BasicSearch {
            results: match string {
                Ok(s) => s,
                Err(e) => e.to_string(),
            },
        };
    }
}

impl Request {
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
                returning list of titles for names matching substrings to provided input..."
            );
            let single_author = format!("author/{}/title", &a.trim_right());
            &self.url.push_str(&single_author);
        } else if author.is_none() && title.is_none() {
            println!("No author or title given... \nreturning list of authors...");
            let authors = format!("author");
            &self.url.push_str(&authors);
        } else if author.is_some() && title.is_some() {
            println!("searching for substring matches by author name and work title given...");
            let author_title = format!("author,title/{};{}", &a.trim_right(), &t.trim_right());
            &self.url.push_str(&author_title);
        } else if author.is_none() && title.is_some() {
            let single_title = format!("title/{}", &t.trim_right());
            &self.url.push_str(&single_title);
        }
        println!("request string: {:?}", &self.url);
        Request { url: self.url.clone() }
    }
}

pub fn get_response(req: Request) -> reqwest::Result<(reqwest::Response)> {
    //let mut author_names: AuthorsList = Vec::new(String::new());
    let res = reqwest::get(&req.url)?;
    Ok((res))
}

pub fn branch_eval(resp: reqwest::Result<reqwest::Response>) -> reqwest::Result<(serde_json::Value)> {
    if resp.is_ok() {
        let mut json_val: serde_json::Value = resp.unwrap().json()?; 
        match &json_val {
            &serde_json::Value::Array(ref arr) => {
                println!("got Array!");
                let msg: String = serde_json::to_string_pretty(&arr.clone()).unwrap();
                println!("Array: {}", &msg);
            }
            &serde_json::Value::Object(ref obj) => {
                println!("got Object!");
                let msg: String = serde_json::to_string_pretty(&obj.clone()).unwrap();
                println!("Object: {}", &msg);
            }
            _ => {
                println!("got... something else!");
                let msg: String = serde_json::to_string_pretty(&json_val.clone()).unwrap();
                println!("something else: {}", &msg);
            }
        }
        return Ok((json_val.to_owned()));
    } else {
        return Err(resp.unwrap_err());
    }
}
pub fn serialize(
    mut resp: reqwest::Result<reqwest::Response>,
) -> reqwest::Result<(serde_json::Value)> {
    if resp.is_ok() {
        let branch = branch_eval(resp)?;
        let json_val: serde_json::Value = branch;

        return Ok((json_val));
    } else {
        return Err(resp.unwrap_err());
    }
}



impl LinesFeeder {
    pub fn new() -> LinesFeeder {
        let arc_mut_vec: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        return LinesFeeder {
            queue: arc_mut_vec,
        }
    }

    pub fn add_lines(&mut self, mut poem: Poem) -> Result<LinesFeeder, Error> {
        let mut queued = match self.queue.lock() {
            Ok(vec)=>vec,
            Err(e) =>  e.into_inner(),
        };
        for line in poem.lines.iter() {
            queued.deref_mut().push(line.clone());
        }
        println!("total lines stored: {:?}", queued.len());
        return Ok((self.clone()));
    }
}

pub fn pretty_print(res: reqwest::Result<(serde_json::Value)>, mut feeder: LinesFeeder) -> serde_json::Result<(String)> {
    //this if allows us to unwrap res safely. If res.is_ok() {...} will only execute the block if it returns with an Ok result--which is what we'd be unwrapping in the block that follows
    if res.is_ok() {
        //vv here vv
        let res = res.unwrap();
        //now we need to make copies of the unwrapped result, since the result type itself does not implement the clone/copy traits we cannot do this earlier
        let j_string = serde_json::to_string_pretty(&res.clone())?;
        //if we've returned an object, our pretty print function will return Null for the selection we'd like to print--which we don't want, so we create a reference to an indexed value if the return is an object, reading the 0th element (the first element) in the object.
        let check_obj: serde_json::Value = serde_json::from_str(&j_string.clone())?;
        //because of how indexing works in Rust, we need to make index a "ref" to inspect it instead of binding it with "let". This prevents the value of j_string/check_obj from being mutated when we do evaluate it.
        let ref index = match &check_obj {
            &serde_json::Value::Object(ref obj) => check_obj[0].to_owned(),
            &serde_json::Value::Array(ref arr) => arr[0].to_owned(),
            &_ => check_obj[0].to_owned(),
        };
        //we know that if we've returned an object, there should be enough fields to represent a "poem" type, so we go ahead and deserialize into that below
        let mut p: Poem = serde_json::from_value(index.clone().to_owned())?;
        
        //then we know we want the lines, so to access a property/field of an instantiated type we use standard dot notation, so var.property (in this case, p.lines)
        println!("got lines! {:?}", &p.lines);
        feeder.add_lines(p);
        // println!("json from pretty_print(): {}", &j_string);
        return Ok((j_string));
    } else {
        println!("something went wrong in pretty_print()");
        return Ok((res.unwrap_err().to_string()));
    }
}
