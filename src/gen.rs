//!Gen Module
//!This file contains behaviors and functions critical to text generation
// #[macro_use]
// extern crate serde_json;

use flavor;
use markov::Chain;
use poems::Poem;
use poems::{AuthorsList, WorksList};
use std::error::Error;
use util;
// use serde_json;

use textwrap::termwidth;

use colored::*;

pub fn seed_and_generate(chain: &Chain<String>, lines_read: usize) -> &Chain<String> {

    let width = termwidth() - 12;

    let mut poem_storage: Vec<String> = Vec::new();
    let mut poem = Poem::new();

    // let name_error: Name = Name::default();
    // let title_error: Work = Work::default();
    //     println!("  |{:^<1$}|", "^", width + 6);

    // println!("DEFAULT NAME: {} {} {}", name_error.first, name_error.middle, name_error.last);
    //     println!("  |{:v<1$}|", "v", width + 6);

    // let title_error: Work = Work { title: String::from("\"A Tale of Error and Woe\"") };
    let gen_name: Name = Name::new().from_list().unwrap_or_default();
    let gen_work: Work = Work::new().from_list().unwrap_or_default();
    let author_fmt = format!(
        "{} {} {}",
        &gen_name.first,
        &gen_name.middle,
        &gen_name.last
    );
    poem.title = gen_work.title;
    poem.author = author_fmt.clone();
    flavor::bard_intro();
    println!("          \"{}, the BARD is here!\"!\n", &poem.author);
    println!(
        "{}",
        "---------------------------------------------------------------------".yellow()
    );
    println!("\n     The bard approaches... and queries...\n    \"Now then, what's this?\"\n");
    flavor::lines_prompt();
    if util::read_y_n() {
        println!("\n     \"Splendid! How many lines should I write?\"\n");
        let num = util::read_int();
        poem.linecount = num as i64;
        println!(
            "\n\n     \"That should do it!\" the bard exclaims. 
                The lights dim--the show begins!\n\n"
        );
        // println!("  |{:-<1$}|", "-", width + 6);
        for line in chain.str_iter_for(num as usize) {
            poem_storage.push(line);
        }
        poem.lines = poem_storage.clone();

        poem.print();
    } else if &lines_read > &50 {
        poem.linecount = 50;
        println!("|{:-<1$}|", "-", width + 6);

        println!(
            "\n\n     \"..although there is virtue in moderation,\" says the bard, 
                \"50 lines it is!\"\n\n"
        );
        println!(
            "\n\n     \"Very well then!\" says the bard. The lights dim--the show begins!\n\n"
        );
        // println!("|{:=<1$}|", "-", width + 6);
        for line in chain.str_iter_for(poem.linecount as usize) {
            poem_storage.push(line);
        }
        poem.lines = poem_storage.clone();
        poem.print();
    } else {
        println!(
            "\n\n     \"Very well then!\" says the bard. The lights dim--the show begins!\n\n"
        );
        poem.linecount = lines_read as i64;
        for line in chain.str_iter_for(lines_read) {
            poem_storage.push(line);
        }
        poem.lines = poem_storage.clone();

        poem.print();
    }
    println!(
        "{}",
        "    Good show! Would you like to save the poem and author to poems.txt?".yellow()
    );
    if util::read_y_n() {
        util::write_poem_to_file(poem.lines, poem.author, poem.title);
    } else {
        println!("    Maybe next time we'll make the cut!");
    }
    return &chain;
}

pub struct Work {
    title: String,
}
impl Work {
    pub fn new() -> Work {
        Work { title: String::new() }
    }
    pub fn from_list(mut self: Self) -> Result<Work, Box<Error>> {
        let list = WorksList::new();
        let mut gen = Chain::new();
        let mut titles_iter = list.titles.into_iter();
        while let Some(title) = titles_iter.next() {
            gen.feed_str(&title);
        }
        let new_title = gen.generate_str();
        self.title.push_str(&new_title);
        return Ok(self);
    }
}
impl Default for Work {
    fn default()-> Work {
        let default: Work = Work {
            title: "A Tale of Woe and Panic".to_string()
        };
        return default
    }
}
pub struct Name {
    first: String,
    middle: String,
    last: String,
}
use std::default::Default;
impl Default for Name {
    fn default()-> Name {
        let default: Name = Name {
            first: "Sir".to_string(),
            middle: "Error".to_string(),
            last: "The Unwrapped None".to_string()
        };
        return default
    }
}
impl Name {
    pub fn new() -> Name {
        Name {
            first: String::new(),
            middle: String::new(),
            last: String::new(),
        }
    }
    pub fn from_list(mut self: Self) -> Result<Name, Box<Error>> {
        let names: AuthorsList = util::get_authors();
        let mut first_name: Chain<String> = Chain::new();
        let mut last_name: Chain<String> = Chain::new();
        let mut m_name: Chain<String> = Chain::new();
        for full_name in names.authors.into_iter() {
            let mut single_name = full_name.as_str().split(" ");
            if let Some(f_n) = single_name.next() {
                // let f_n = single_name.next().unwrap();
                // println!("got first name! \n{}", f_n);
                first_name.feed_str(f_n);
                if let Some(m_n) = single_name.next() {
                    // let l_n = single_name.next().unwrap();
                    m_name.feed_str(m_n);
                    // println!("got last name! \n{}", l_n);
                    if let Some(l_n) = single_name.next() {
                        last_name.feed_str(l_n);

                    }
                }
            }
        }
        let new_first = first_name
            .generate_str()
            .split(" ")
            .next()
            .unwrap()
            .to_owned();
        let new_middle = m_name.generate_str().split(" ").next().unwrap().to_owned();
        let new_last = last_name
            .generate_str()
            .split(" ")
            .next()
            .unwrap()
            .to_owned();
        self.first.push_str(&new_first);
        self.middle.push_str(&new_middle);
        self.last.push_str(&new_last);

        let new_name: Name = Name {
            first: self.first.clone(),
            middle: self.middle.clone(),
            last: self.last.clone(),
        };

        return Ok(new_name);
    }

    pub fn from_name_string(s: &str) -> Name {
        let mut name = Name::new();
        let mut names = s.split(" ");
        name.first.push_str(names.next().unwrap_or("Sir Error"));
        name.last.push_str(
            names.next().unwrap_or("The Unwrapped None"),
        );
        return name;
    }
}
