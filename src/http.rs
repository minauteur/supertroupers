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
use serde;
use util;
use text_io;
use std::collections::HashMap;
//use std::ops::Try;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    authors: Vec<String>,
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
    pub fn author_title() -> BasicSearch {
        println!("Search for an Author?");
        // let mut author = String::new();
        let author = util::read_in_ln();
        // match io::stdin().read_line(&mut author) {
        //     Ok(n) => {
        //         if author == "\n".to_string() {
        //             println!("No author entered.");
        //         } else {
        //             println!("Searching author: {}", author);
        //         }
        //     }
        //     Err(error) => println!("error: {}", error),
        // };
        println!("and a title?");
        let title = util::read_in_ln();
        // let mut title = String::new();
        // let t = match io::stdin().read_line(&mut title) {
        //     Ok(n) => {
        //         if title == "\n".to_string() {
        //             println!("No title entered.");
        //         } else {
        //             println!("Searching title: {}", title);
        //         }
        //     }
        //     Err(error) => {
        //         println!("error: {}", error);
        //     }
        // };
        // let author: Option<String> = match author.as_ref() {
        //     "\n" => {
        //         println!("read newline, author == None");
        //         None
        //     }
        //     string => Some(string.to_string()),
        // };
        println!("checking author value... author == {:?}", author);
        println!("checking title value... title == {:?}", title);

        let mut req: Request = RequestBuilder::new().with_params(author, title);
        let resp = get_response(req);
        let serialized = serialize(resp);
        // let poem = get_lines(serialized);
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
            println!("No title given... \nreturning list of titles for names matching substrings to provided input...");
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
pub fn resp_value_branch(mut response: reqwest::Response) -> reqwest::Result<(String)> {
    let data: serde_json::Value = response.json()?;
    let map: HashMap<String, Vec<String>> = serde_json::from_value(data.clone()).unwrap();
    if map.contains_key("authors") {
        let v = map.get("authors").unwrap();
        let list: AuthorsList = AuthorsList {
            authors: v.to_owned(),
        };
        println!("\"authors\": \n{:?}", &list.authors);
        return Ok((list.authors.join(" ")));
    } else { 
        return Ok((serde_json::to_string_pretty(&data).expect("couldn't unwrap text from Object!")));
    }
}
pub fn serialize(
    mut resp: reqwest::Result<reqwest::Response>,
) -> reqwest::Result<(serde_json::Value)> {
    if resp.is_ok() {
        let data: serde_json::Value = resp.unwrap().json()?;
        match &data {
            &serde_json::Value::Array(ref x) => {
                println!("got Array!");
            }
            &serde_json::Value::Object(ref y) => {
                println!("got Object!")
            }
            _=> println!("something else!"),
        }
        // let d_str: String = serde_json::to_string(&data.clone()).unwrap();
        // let poem: Poem = serde_json::from_str(&d_str).unwrap();
        // println!("no problem, here are the lines! \n{:?}", &poem);
        // let ref index = &data[0];
        // println!("here is our serde value at index 0: {:?}", data[0]);
        return Ok((data));
    } else {
        return Err(resp.unwrap_err());
    }
}

pub fn pretty_print(res: reqwest::Result<(serde_json::Value)>) -> serde_json::Result<(String)> {
    if res.is_ok() {
        let res = res.unwrap();
        let j_string = serde_json::to_string_pretty(&res.clone())?;
        let check_obj: serde_json::Value = serde_json::from_str(&j_string.clone())?;
        let ref index = check_obj[0];
        let p: Poem = serde_json::from_value(index.to_owned())?;
        println!("got lines! {:?}", p.lines);
        println!("json from pretty_print(): {}", &j_string);
        return Ok((j_string));
    } else {
        println!("something went wrong in pretty_print()");
        return Ok((res.unwrap_err().to_string()));
    }
}
