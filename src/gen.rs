//!Gen Module
//!This file contains behaviors and functions critical to text generation
use std::io::Write;
use std::fs::OpenOptions;

use markov::Chain;

use poems::{AuthorsList, WorksList};
use std::error::Error;
use util;
// use gen;
// #[cfg(feature = "term_size")]
// #[cfg(feature = "hyphenation")]
//  #[feature(hyphenation, term_size)]
use textwrap::{Wrapper, WordSplitter, HyphenSplitter, termwidth};
use hyphenation::*;
use hyphenation;
use colored::*;


// #[cfg(feature = "term_size")]
// #[cfg(feature = "hyphenation")] 
// #[feature = "term_size"]
pub fn seed_and_generate(chain: &Chain<String>, lines_read: usize) -> &Chain<String> {
    // let mut chain = Chain::new();
    
    let corpus = hyphenation::load(Language::English_US).unwrap();
    let width = termwidth()-12;
    let indent = format!("{}","              ".clear());
    let wrapper = Wrapper::with_splitter(width, corpus).break_words(true).subsequent_indent(&indent);
        // .subsequent_indent(indent);
    let mut poem_storage: Vec<String> = Vec::new();
    let mut author_storage = String::new();

    let name_error: Name = Name {
        first: String::from("Sir Erronaeus,"),
        last: String::from("The Unwrapp-ed None"),
    };
    let title_error: Work = Work {
        title: String::from("\"A Tale of Error and Woe\""),
    };
    let gen_name: Name = Name::new().from_file().unwrap_or(name_error);
    let gen_work: Work = Work::new().from_file().unwrap_or(title_error);
    let author_fmt = format!("{} {}", &gen_name.first, &gen_name.last).bold();

    flavor_generator();
    println!("          \"{}, the BARD is here!\"!\n", &author_fmt);
    
    author_storage.push_str(&author_fmt);

    println!(
        "{}",
        "---------------------------------------------------------------------".yellow()
    );
    println!("\n     The bard approaches... and queries...\n    \"Now then, what's this?\"\n");
    // for string in &seed_store {
    //     chain.feed_str(string);
    // }
        flavor_lines_prompt();
        if util::read_y_n() {
            println!("\n     \"Splendid! How many lines should I write?\"\n");
            let num = util::read_int();
            println!(
                "\n\n     \"That should do it!\" the bard exclaims. The lights dim--the show begins!\n\n"
            );
            println!(
                "  |{:=<1$}|", "=", width + 6,
            );
            let title = format!("  a poem: \"{}\"", wrapper.fill(gen_work.title.trim()).green());
            println!("  |    {:<1$}  |", title, width+9);

            println!("  |{:-<1$}|", "-", width + 6);
            for line in chain.str_iter_for(num as usize) {
                if !line.is_empty() {
                    let gen =format!("{}", wrapper.fill(&chain.generate_str()));
                    // let ex = format!("{}", &gen, width);
                    // if line.len() < 75 {
                    //     while line.len() < 75 {
                    //         line.push_str(" ");
                    //     }
                    // }
                        println!("  |    {:<1$}|", gen, width+2);
                        poem_storage.push(line);
                    
                } else {
                    println!("  |{:<1$}|","      ----------".to_string(), width+6);
                }
            }
            // wrap_example(width, poem_storage.clone());
        } else if &lines_read > &50 {
            println!("|{:-<1$}|", "-", width + 6);

            println!(
                "\n\n     \"..although there is virtue in moderation,\" says the bard, \"50 lines it is!\"\n\n"
            );
            println!(
                "\n\n     \"Very well then!\" says the bard. The lights dim--the show begins!\n\n"
            );
            println!(
                "|{:=<1$}|", "-", width + 6
            );
            let title = format!("  |  a poem: \"{}\"|", wrapper.fill(gen_work.title.trim()).green(),);            
            // println!("|  A Poem: \"{}\"", wrapper.fill(gen_work.title.trim()));
            println!("  |  {:1$}  |", title, width+6);
            println!(
                "|{:-<1$}|", "-", width + 6
            );
            for line in chain.str_iter_for(50) {
                if !line.is_empty() {
                    let line = format!("{}", chain.generate_str());
                    println!("|   {}", wrapper.fill(&line).bright_green());
                    poem_storage.push(line);
                } else {
                    println!("|{:<1$}|","      ----------".to_string(), width+6);
                }
            }
        } else {
            println!(
                "\n\n     \"Very well then!\" says the bard. The lights dim--the show begins!\n\n"
            );
            println!("|{:=<1$}|", "-", width + 6,);

                        println!("|  a poem: \"{}\"", wrapper.fill(gen_work.title.trim()));
            println!("|{:-<1$}|", "-", width + 6,);

            for line in chain.str_iter_for(lines_read) {
                if !line.is_empty() {
                    let line = format!("{}", chain.generate_str());
                    println!("|   {:^1$}", wrapper.fill(&line).bright_green(), width);
                    poem_storage.push(line);

                } else {
                    println!("|                    -------------");

                }
            }
        }
        let mut a = gen_name.first.clone();
        a.push(' ');
        a.push_str(&gen_name.last);
        let a_s = format!("  author: {}", wrapper.fill(a.trim()).purple());
            println!("  |{:-<1$}|", "-", width + 6,);
        println!("  |    {:1$}  |", a_s, width+9);                                
        println!("  |{:=<1$}|", "=", width + 6,);
        // wrap_example(width, poem_storage);
    println!(
        "{}",
        "    Good show! Would you like to save the poem and author to poems.txt?".yellow()
    );
    if util::read_y_n() {
        write_poem_to_file(poem_storage, author_storage, gen_work.title);
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
        Work {
            title: String::new()
        }
    }
    pub fn from_file(mut self: Self) -> Result<Work, Box<Error>> {
        let list = WorksList::new();
        let mut gen = Chain::new();
        let mut titles_iter = list.titles.into_iter();
        while let Some(title) = titles_iter.next() {
                gen.feed_str(&title);
                // if let Some(word_2) = single_word.next() {
                //     gen.feed_str(word_2);
                //     if let Some(word_3) = single_word.next() {
                //         gen.feed_str(word_3);
                //         if let Some(word_4) = single_word.next() {
                //             gen.feed_str(word_4);
                //             if let Some(word_5) = single_word.next() {
                //                 gen.feed_str(word_5);
                //             }
                //         }
                //     }
                // }
            }
        let new_title = gen.generate_str();
        self.title.push_str(&new_title);
        return Ok(self)
    }
}

pub struct Name {
    first: String,
    // middle: String,
    last: String,
}
impl Name {
    pub fn new() -> Name {
        Name {
            first: String::new(),
            // middle: String::new(),
            last: String::new(),
        }
    }
    pub fn from_file(mut self: Self) -> Result<Name, Box<Error>> {
        let names: AuthorsList = util::read_authors_from_file().expect("error reading from file!");
        let mut first_name: Chain<String> = Chain::new();
        let mut last_name: Chain<String> = Chain::new();

        for full_name in names.authors.into_iter() {
            let mut single_name = full_name.as_str().split(" ");
            if let Some(f_n) = single_name.next() {
                // let f_n = single_name.next().unwrap();
                // println!("got first name! \n{}", f_n);
                first_name.feed_str(f_n);
                if let Some(l_n) = single_name.next() {
                    // let l_n = single_name.next().unwrap();
                    last_name.feed_str(l_n);
                    // println!("got last name! \n{}", l_n);
                }
            }
        }
        let new_first = first_name
            .generate_str()
            .split(" ")
            .next()
            .unwrap()
            .to_owned();
        let new_last = last_name
            .generate_str()
            .split(" ")
            .next()
            .unwrap()
            .to_owned();
        self.first.push_str(&new_first);
        self.last.push_str(&new_last);

        let new_name: Name = Name {
            first: self.first.clone(),
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

pub fn write_poem_to_file(poem: Vec<String>, author: String, title: String) {

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("poems.txt")
        .unwrap();
    // let mut writer = BufWriter::new(&mut file);
    if let Err(e) = writeln!(file, "\r\n") {
        println!("{}", e);
    }
    if let Err(e) = writeln!(file, "  \"{}\"\r\n\r\n", title) {
        println!("{}", e);
    }
    for line in poem {
        if let Err(e) = writeln!(file, "    {}\r\n", line) {
            println!("{}", e);
        }
    }
    if let Err(e) = writeln!(file, "\r\n    --{}\r\n", author) {
        println!("{}", e);
    }
    if let Err(e) = writeln!(
        file,
        "\r\n------------------------------------------------------\r\n"
    )
    {
        println!("{}", e);
    }
    println!(
        "{} See poems.txt in your supertroupers folder to view output.",
        " Success!".green()
    );
}
pub fn flavor_generator() {
    println!("{}", "  from the mist...".clear());
    println!("{}", "      ~~~~~".purple());
    println!("{}", "         ~~~~~~~~~".bright_blue());
    println!("{}", "           ~~~~~~~~~~~".blue());
    println!("{}", "          a shadow nears...".clear());
    println!("{}", "              ~~~~~~~~~~~~".blue());
    println!("{}", "                  ~~~~~~~".purple());
    println!("{}", "                 ~~~~".blue());
    println!(
        "{}",
        "  no, not death--the figure of a BARD appears!".clear()
    );
    println!("{}", "             ~~~~~".bright_blue());
    println!("{}", "           ~~~~~~~~~".blue());
    println!("{}", "             ~~~~~~~~~~~".bright_blue());
    println!(
        "{}",
        "        \"I fear death less, perhaps...\" you think,s\n            \"than being bored to tears!\""
    );
    println!("{}", "               ~~~~~~~~~~~~".purple());
    println!("{}", "                 ~~~~~~~".blue());
    println!("{}", "                 ~~~~".bright_blue());
    println!(
        "{}",
        "              hurry though as you might,\n               before you drain your beer"
    );
    println!("{}", "           an apprehensive patron cries--");

}
pub fn flavor_lines_prompt() {
            println!(
            "\n     \"Quite a bit of material, I think!\" \n      \"Should we keep the poem to a set number of lines?\"\n"
        );
        println!(
            "{}",
            "  |---------------------------------------------------------------------------|"
                .bright_yellow()
        );
        println!(
            "{}{}{}{}{}{}{}",
            "  |".bright_yellow(),
            "  ENTER:".clear(),
            " N".red(),
            " or ".clear(),
            "n".red(),
            " to generate lines equal to the number of total lines read".clear(),
            "  |".bright_yellow()
        );
        println!(
            "{}{}{}{}{}{}{}",
            "  |".bright_yellow(),
            "  ENTER:".clear(),
            " Y".green(),
            " or ".clear(),
            "y".green(),
            " to specify the number of lines to generate".clear(),
            "                 |".bright_yellow()
        );
        println!(
            "{}",
            "  |---------------------------------------------------------------------------|"
                .bright_yellow()
        );
}
pub fn wrap_example(width: usize, lines: Vec<String>) {
        let example = lines.join("\n");
    let mut prev_lines = vec![];
    let mut wrapper = Wrapper::new(width).subsequent_indent("        ");
    let lines = wrapper.wrap(&example);
        if lines != prev_lines {
            let title = format!(" Width: {} ", width);
            println!(".{:-^1$}.", &title, width + 2);
            for line in &lines {
                println!("| {:^1$} |", &line, width);
            }
        prev_lines = lines;
    }
}
