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

pub struct RequestBuilder {
    author: Option<String>,
    title: Option<String>,
}

pub struct Request {
    url: PathBuf,
}

impl RequestBuilder {
    pub fn new() -> Request {
        let mut request = PathBuf::from("http://poetrydb.org/");
        Request {
            url: request,
        }
    }
    pub fn new_with_params(author: Option<String>, title: Option<String>) -> Request {
        let mut request = RequestBuilder::new();
        let a: String = match author {
            Some(auth_name) => auth_name,
            None => String::from("author"),
        };
        let t: String = match title {
            Some(text_name) => format!(",{}", text_name),
            None => String::from(""),
        };
        request.url.push(&a);
        request.url.push(&t);
        Request {
            url: request.url,
        }
    }
    pub fn get_authors() -> AuthorsList {
             //let mut author_names: AuthorsList = Vec::new(String::new());
        let author_names: HashMap<String, Vec<String>> = reqwest::get("http://poetrydb.org/author").unwrap().json().unwrap();
        AuthorsList {
            authors: author_names,
        }
    }
}