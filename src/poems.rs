//!Poems houses the structs and implementations for deserializing and
//! storing Poem data retrieved from poetrydb API requests
use serde_json::{self, Value};
use textwrap::{Wrapper, termwidth, };
use textwrap::wrap_iter;
use hyphenation::*;
use hyphenation;
use util;
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksList {
    pub titles: Vec<String>,
}

impl WorksList {
    pub fn new() -> WorksList {
        let default: WorksList = WorksList {
            titles: Vec::new(),
        };
        let list: WorksList = util::read_titles_from_file().unwrap_or(default);
        return list
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Poem {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
    pub line_count: i64,
}

impl Poem {
    pub fn new() -> Poem {
        Poem {
            title: String::new(),
            author: String::new(),
            lines: Vec::new(),
            line_count: 0,
        }
    }
    pub fn from_value(mut self, json: &Value) -> Result<Poem, serde_json::Error> {
        if let Some(ref lines) = json.get("lines") {
            &self.get_lines(lines)?;
            // println!("got lines!");
        } else {
            // println!("no lines found!");
        }
        if let Some(ref author) = json.get("author") {
            &self.get_author(author)?;
            // println!("got author!");
            
        } else {
            // println!("no author name found!");
        }
        if let Some(ref title) = json.get("title") {
            &self.get_title(title)?;
            // println!("got title!");
        } else {
            // println!("no title found!");
        }
        if let Some(ref l_c) = json.get("linecount") {
            &self.get_count(l_c)?;
            // println!("got line count!");
            
        } else {
            // println!("no linecount found!");
        }
        return Ok((self));

    }
    pub fn print(&self) -> Self {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let width = termwidth()-12;
        let author = format!("  author: {}", self.author.purple());
        let title = format!(" \"{}\"", self.title);
        let poem = self.lines.join("\n");
        let wrapper = Wrapper::with_splitter(width, corpus).break_words(true).subsequent_indent("        ");
        
        println!("  |{:=<1$}|", "=", width + 6);
        for t_line in wrap_iter(&title, width) {
            let t_fmt = format!("{:<1$}", wrapper.fill(&t_line), width-9);
            println!("  |  a poem:{:<1$}    |", &t_fmt.green(), width-7);
        }
        println!("  |{:-<1$}|", "-", width + 6);
        
        // for line in poem.lines().into_iter() {
        for line in wrap_iter(&poem, width) {
            let formatted = format!("{:<1$}", &line, width-9);
            // let formatted = format!("{}", &line);
            
            // let formatted = format!("{:<1$}", fill(&line, width-9), width-9);
            for line in formatted.lines() {
                println!("  |   {:<1$}   |", &line.bright_green(), width);
            }
        }
        println!("  |{:-<1$}|", "-", width + 6);
        println!("  |{:<1$}      |", wrapper.fill(&author), width+9);
        println!("  |{:=<1$}|", "=", width + 6);
        return self.to_owned();
    } 
    fn get_lines(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.lines = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
    fn get_author(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.author = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
    fn get_title(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.title = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
    fn get_count(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.line_count = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
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
        return list
        
        }
}

