//!Gen Module
//!This file contains behaviors and functions critical to text generation
extern crate rand;
// extern crate term;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;
use markov::Chain;
static LOC_SEED_DIR: &'static str = "names.json";
use http::AuthorsList;
use std::error::Error;
use util;
use gen;
use serde_json;

pub struct Name {
    first: String,
    // middle: String,
    last: String
}
impl Name {
    pub fn new() -> Name {
        Name {
            first: String::new(),
            // middle: String::new(),
            last: String::new(),
        }
    }

    pub fn from_file(mut self: Self)-> Result<gen::Name, Box<Error>> {
        let mut names = read_authors_from_file().expect("something went wrong reading from file!");
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
        // println!("!?But who will claim thes words?!");
        // println!("??Does the creator have a name??");
        // println!("         ...........");
        // println!("          .........");
        // println!("           .......");
        // println!("            .....");
        // println!("             ...");
        // println!("              V");
        // println!("???And can it claim them for itself???");

        let new_first = first_name.generate_str().split(" ").next().unwrap().to_owned();
        let new_last = last_name.generate_str().split(" ").next().unwrap().to_owned();
        self.first.push_str(&new_first);
        self.last.push_str(&new_last);

        let new_name: Name =  Name {
            first: self.first.clone(),
            last: self.last.clone(),
        };


        return Ok(new_name);
    }
    pub fn from_name_string(s: &str)-> Name {
        let mut name = Name::new();
        let mut names = s.split(" ");
        name.first.push_str(names.next().unwrap_or("Sir Error"));
        name.last.push_str(names.next().unwrap_or("The Unwrapped None"));
        return name
    }   
}

fn read_authors_from_file() -> Result<AuthorsList, Box<Error>> {
    // Open the file in read-only mode.
    let path = PathBuf::from(LOC_SEED_DIR);
    let file = File::open(&path)?;

    // Read the JSON contents of the file as an instance of `AuthorsList`.
    let list: AuthorsList = serde_json::from_reader(file)?;

    // Return the `List`.
    Ok(list)
}




pub fn seed_and_generate(seed_store: Vec<String>) {
    let mut chain = Chain::new();
    let mut poem_storage: Vec<String> = Vec::new();
    let mut author_storage = String::new();
        //     let mut a_c = Chain::new();
        
        // a_c.feed_str("William ").feed_str("Shakespeare ").feed_str("Johnson ")
        //     .feed_str("Betty ").feed_str("Emily ").feed_str("Mary ").feed_str("Shelley ").feed_str("Blake ");
        // let mut name_string = String::new();
        // for name in a_c.str_iter_for(3) {
        //     let x = format!("{}", a_c.generate_str());
        //     name_string.push_str(&x);
        // }
        let error: Name = Name {
            first: String::from("Sir Erronaeus,"),
            last: String::from("The Unwrapp-ed None"),
        };
        let gen_name: Name = Name::new().from_file().unwrap_or(error);
        println!("     from the mist...");
        println!("              ~~~~~");
        println!("             ~~~~~~~~~");
        println!("               ~~~~~~~~~~~");
        println!("              a shadow nears...");
        println!("                  ~~~~~~~~~~~~");
        println!("                      ~~~~~~~");
        println!("                     ~~~~");
        println!("      no, not death--the figure of a BARD appears!");
        println!("                 ~~~~~");
        println!("               ~~~~~~~~~");
        println!("                 ~~~~~~~~~~~");
        println!("            \"I fear death less, perhaps...\" you think,\n            \"than being bored to tears!\"");
        println!("                   ~~~~~~~~~~~~");
        println!("                     ~~~~~~~");
        println!("                     ~~~~");
        println!("                  hurry though as you might,\n               before you drain your beer");
        println!("               an apprehensive patron cries--");
        println!("        \"{} {}, the BARD is here!\"!\n", gen_name.first, gen_name.last);
        let author_fmt = format!("{} {}", &gen_name.first, &gen_name.last);
        author_storage.push_str(&author_fmt);
    for string in seed_store.clone() {
        chain.feed_str(&string);
    }
    println!("---------------------------------------------------------------------");
    println!("\n     The bard approaches... and queries...\n    \"Now then, what's this?\"\n");
    if seed_store.len() > 30 {
        println!("\n     \"Quite a bit of material, I think!\" \n      \"Should we keep the poem to a set number of lines?\"\n");
        println!("  |---------------------------------------------------------------------------|");
        println!("  |  ENTER: N or n to generate lines equal to the number of total lines read  |"); 
        println!("  |  ENTER: Y or y to specify the number of lines to generate                 |");
        println!("  |---------------------------------------------------------------------------|");
    if util::read_y_n() {
        println!("\n     \"Splendid! How many lines should I write?\"\n");
        let num = util::read_int();
        println!("\n\n     \"That should do it!\" the bard exclaims. The lights dim--the show begins!\n\n");
        println!("|============================================================================|");

        for line in chain.str_iter_for(num as usize) {
            if !line.is_empty() {
                let line = format!("{}", chain.generate_str());
                println!("|   {}", line);
                poem_storage.push(line);
            } else {
                println!("|------------------------------------------------------------------------");
            }

        }
    }
    println!("|============================================================================|
              |        author: {} {}
              |========================================================|", gen_name.first, gen_name.last);
    } else {
        println!("|--------------------------------------------------------");
        println!("\n\n     \"Very well then!\" says the bard, and without wait the show begins!\n\n");
        for line in chain.str_iter_for(seed_store.len()) {
            if !line.is_empty() {
                let line = format!("{}", chain.generate_str());
                println!("|   {}", line);                
                poem_storage.push(line);

            } else{
                println!("|--------------------------------------------------------");
                
            }
        }
    println!("|========================================================================|
              |        author: {} {}
              |=======================================================|",
              gen_name.first, gen_name.last);
    }
    println!("    Good show! Would you like to save the poem and author to poems.txt?");
    if util::read_y_n() {
        write_poem_to_file(poem_storage, author_storage);
    } else {
        println!("    Maybe next time we'll make the cut!");
    }
}
pub fn write_poem_to_file(poem: Vec<String>, author: String) {
    
        let mut file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("poems.txt")
        .unwrap();
    // let mut writer = BufWriter::new(&mut file);
    if let Err(e) = writeln!(file, "\r\n") {
        println!("{}", e);
    }
    for line in  poem {
        if let Err(e) = writeln!(file, "    {}\r\n", line) {
            println!("{}", e);
        }
    }
    if let Err(e) = writeln!(file, "\r\n    --{}\r\n", author) {
        println!("{}", e);
    }
    if let Err(e) = writeln!(file, "\r\n------------------------------------------------------\r\n") {
        println!("{}", e);
    }
    println!("     Success! see poems.txt in your supertroupers folder to view output.");
}
