//!Poems houses the structs and implementations for deserializing and
//! storing Poem data retrieved from poetrydb API requests
use serde_json::{self, Value};
use textwrap::{fill, termwidth, Options};
// use textwrap::wrap_iter;
use crate::util;
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksList {
    pub titles: Vec<String>,
}

impl WorksList {
    pub fn new() -> WorksList {
        let default: WorksList = WorksList { titles: Vec::new() };
        let list: WorksList = util::read_titles_from_file().unwrap_or(default);
        return list;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Poem {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
    pub linecount: String,
    pub linenumber: i64,
}

impl Poem {
    pub fn new() -> Poem {
        Poem {
            title: String::new(),
            author: String::new(),
            lines: Vec::new(),
            linecount: "0".to_string(),
            linenumber: 0,
        }
    }
    pub fn from_value(mut self, json: &Value) -> Result<Poem, serde_json::Error> {
        if let Some(lines) = json.get("lines") {
            self.get_lines(&lines)?;
            println!("got lines {}", &lines);
        } else {
            println!("no lines found!");
        }
        if let Some(author) = json.get("author") {
            self.get_author(&author)?;
            println!("got author: {}", &self.author);
        } else {
            // println!("no author name found!");
        }
        if let Some(ref title) = json.get("title") {
            self.get_title(&title)?;
            println!("got title {}", &title);
        } else {
            // println!("no title found!");
        }
        if let Some(l_c) = json.get("linecount") {
            self.get_count(&l_c)?;
            println!("got line count {}", &l_c);
            let vs = l_c.as_str().expect("value exists");
            println!("string: {:?}", &vs);
            let int_val = vs.parse::<u32>().unwrap() as i64;
            println!("int val: {:?}", int_val);
            self.linenumber = int_val;
        } else {
            // println!("no linecount found!");
        }
        println!("poem: {:#?}", &self);
        return Ok(self);
    }
    pub fn print(&self) -> Self {
        if termwidth() < 80 {
            println!("you should resize your terminal to be wider than 80 columns");
            return self.to_owned();
        }
        let width = Options::new(68)
            .initial_indent("  ")
            .subsequent_indent("      ");
        let author = format!(" author: {}", self.author.purple());
        let title = format!(" a poem: \"{}\"", self.title);
        let poem = self.lines.join("\n");

        println!("  |{:=<1$}|", "=", 68 + 6);
        let t_fmt = format!("{}", fill(&title, &width));
        for t_l in t_fmt.lines() {
            println!("  |  {:<1$}    |", &t_l, 68);
        }
        println!("  |{:-<1$}|", "-", 68 + 6);

        for line in poem.lines() {
            let formatted = format!("{}", fill(&line, &width));
            // let formatted =
            // println!("  |   {:<1$}   |", fill(&formatted, width), width);

            // let formatted = format!("{:<1$}", fill(&line, width-9), width-9);
            for line in formatted.lines() {
                println!("  |   {:<1$}   |", &line.bright_green(), 68);
            }
        }
        println!("  |{:-<1$}|", "-", 68 + 6);
        println!("  |{:<1$}      |", fill(&author, width), 68 + 9);
        println!("  |{:=<1$}|", "=", 68 + 6);
        return self.to_owned();
    }
    fn get_lines(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.lines = serde_json::from_value(json.clone())?;
        return Ok(self.to_owned());
    }
    fn get_author(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.author = serde_json::from_value(json.clone())?;
        return Ok(self.to_owned());
    }
    fn get_title(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.title = serde_json::from_value(json.clone())?;
        return Ok(self.to_owned());
    }
    fn get_count(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.linecount = serde_json::from_value(json.clone())?;
        return Ok(self.to_owned());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    pub authors: Vec<String>,
}
impl AuthorsList {
    pub fn new() -> AuthorsList {
        let default: AuthorsList = AuthorsList {
            authors: Vec::new(),
        };
        let list: AuthorsList = util::read_authors_from_file().unwrap_or(default);
        return list;
    }
}
