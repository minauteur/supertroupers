//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
use std::path::{Path, PathBuf};
use reqwest;
#[macro_use()]
use serde_derive;

use serde_json;
use serde::{Serialize, Deserialize};

use text_io;
use std::collections::HashMap;


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
        Request {
            url: request,
        }
    }
}

pub fn get_response(req: Request) -> serde_json::Value {
    //let mut author_names: AuthorsList = Vec::new(String::new());
    let map: serde_json::Value = reqwest::get(&req.url).unwrap().json().unwrap();
    println!("response contents: {:?}", &map);
    map
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
            let author_title = format!("author,title/{};{}",&a, &t);
            &self.url.push_str(&author_title);
        } else if author.is_none() && title.is_some() {
            let single_title = format!("title/{}", &t);
            &self.url.push_str(&single_title);
        }
        println!("request string: {:?}", &self.url);
        Request {
            url: self.url.clone(),
        }
    }
}