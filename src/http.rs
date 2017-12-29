//!HTTP Module
//!This file contains the necessary logic for making http requests to both
//!poetrydb.org and the phoneme API for serialization
use std::path::{Path, PathBuf};
use reqwest;
use serde_derive;
use serde_json::*;
use serde::{Serialize, Deserialize};

use text_io;



#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    authors: Vec<String>,
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
    pub fn new(a: Option<String>, b: Option<String>) -> Request {
        let mut x = PathBuf::from("http://poetrydb.org/");
        let auth_nm: String = match a {
            Some(a) => a,
            None => String::from("authors"),
        };
        let poem_title: String = match b {
            Some(b) => b,
            None => String::from(",title"),
        };
        x.push(&auth_nm);
        x.push(&poem_title);
        Request {
            url: x,
        }
    }
}