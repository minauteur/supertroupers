//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
 #![feature(try_trait)]
use std::path::{Path, PathBuf};
use reqwest;
#[macro_use()]
use serde_derive;
use std::io;

use serde_json;
use serde::{Serialize, Deserialize};

use text_io;
use std::collections::HashMap;
//use std::ops::Try;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    authors: HashMap<String, Vec<String>>,
}

pub struct AuthorWorks {
    author: String,
    works: Vec<SinglePoem>,
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
    pub fn author_title() -> BasicSearch {
        println!("Search for an Author?");
        let mut author = String::new();
        let a = match io::stdin().read_line(&mut author) {
            Ok(n) => {
                if author == "\n".to_string() {
                    println!("Author not given");
                } else {
                    println!("Read Author: {}", author);
                }
            }
            Err(error) => println!("error: {}", error),
        };
        println!("and a title?");
        let mut title = String::new();
        let t = match io::stdin().read_line(&mut title) {
            Ok(n) => {
                if title == "\n".to_string() {
                    println!("Title not given.");
                } else {
                    println!("Read title: {}", title);
                }
            }
            Err(error) => {
                println!("error: {}", error);
            }
        };
        let author: Option<String> = match author.as_ref() {
            "\n" => {
                println!("read newline, author == None");
                None
            }
            "Test2" => {
                println!("2");
                Some("Test2".to_string())
            }
            string => Some(string.to_string()),
        };
        let title: Option<String> = match title.as_ref() {
            "\n" => {
                println!("read newline, title == None");
                None
            }
            "Test2" => {
                println!("2");
                Some("Test2".to_string())
            }
            string => Some(string.to_string()),
        };
        println!("author == {:?}", author);
        println!("title == {:?}", title);

        let mut req: Request = RequestBuilder::new().with_params(author, title);
        let resp = get_response(req);
        let serialized = serialize(resp);
        let string = pretty_print(serialized);
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
            let single_author = format!("author/{}/title", &a);
            &self.url.push_str(&single_author);
        } else if author.is_none() && title.is_none() {
            let authors = format!("author");
            &self.url.push_str(&authors);
        } else if author.is_some() && title.is_some() {
            let author_title = format!("author,title/{};{}", &a, &t);
            &self.url.push_str(&author_title);
        } else if author.is_none() && title.is_some() {
            let single_title = format!("title/{}", &t);
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

pub fn serialize(
    mut resp: reqwest::Result<reqwest::Response>,
) -> reqwest::Result<(serde_json::Value)> {
    if resp.is_ok() {
        let data: serde_json::Value = resp.unwrap().json()?;
        return Ok((data));
    } else {
        return Err(resp.unwrap_err());
    }
}

pub fn pretty_print(res: reqwest::Result<(serde_json::Value)>) -> serde_json::Result<(String)> {
    if res.is_ok() {
        let j_string = serde_json::to_string_pretty(&res.unwrap())?;
        println!("json from pretty_print(): {}", &j_string);
        return Ok((j_string));
    } else {
        println!("something went wrong in pretty_print()");
        return Ok((res.unwrap_err().to_string()));
    }
}
