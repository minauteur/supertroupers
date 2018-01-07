//!Utilities Module
//!Various Helper functions and error definitions used throughout the project live here

use std::io::{BufReader, BufWriter, Read, Write};
// use std::fmt::Write;
use std::io::BufRead;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use serde_derive;
use serde::{Serialize, Deserialize};
use serde_json::*;

static LOC_SEED_DIR: &'static str = "shakespeare.txt";

#[derive(Serialize, Deserialize)]
pub struct PoetryAPIResp {
    title: String,
    author: String,
    lines: Poem,
}
#[derive(Serialize, Deserialize)]
pub struct Poem {
    linecount: i32,
    lines: Vec<String>,
}

pub fn read_y_n()-> bool {
    loop {
        println!("Continue? Y/N");
        let i = read_in_ln();
        match i {
            Some(s) => {
                match s.trim() {
                    "Y" | "y" => {
                        println!("read Y");                 
                        return true;
                    }
                    "N" | "n" => {
                        println!("read N");
                        return false;
                    }
                    _ => {
                        println!("lets try that again...");
                        continue
                    }
                }
            }
            None => {
                println!("lets try that again, shall we?");
                continue
            }
        }
    }
}

pub fn read_in_ln() -> Option<String> {
    let mut out = String::new();
    let input = match io::stdin().read_line(&mut out) {
        Ok(n) => {
            if out == "\n".to_string() {
                    println!("Nothing entered.");                    
                    return None;
            } else if out == "\r".to_string() {
                println!("Nothing entered.");
                return None;
            } else {
                return Some(out);
            }
        }
        Err(error) => {
            println!("error reading input: {}", error);
            return None;
        }
    };
}

pub fn format_txt() {
    let p = PathBuf::from(&LOC_SEED_DIR);
    let f = File::open(&p).unwrap();
    let file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        let mut b = [0; 4];
        let mut d = [0, 4];
        let mut chars: String = l.chars().collect();
        for c in chars.clone().chars().into_iter() {
            let p: char = '.';
            let r: char = '\n';
            match &c {
                p => {
                    if c == *p {
                        let index = chars.find('.').unwrap_or_default();
                        chars.insert(index, '\n' as char);
                    } else {
                        continue;
                    }
                }
                r => {
                    let index = chars.find('\n').unwrap_or_default();
                    chars.remove(index);
                }
                _ => (),
            }
        }
        writeln!(writer, "{}", chars).unwrap();
        // if num % 4 == 1 {
        //     writeln!(writer, "{}", l).unwrap();
        // }
    }
}


pub fn read_file() {
    let path = PathBuf::from(&LOC_SEED_DIR);
    let txt_src = File::open(&path).unwrap();
    let txt_dest =
        File::create("output.txt").expect("Couldn't create destination file for output!");
    let reader = BufReader::new(&txt_src);
    let mut writer = BufWriter::new(&txt_dest);
    for (num, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut line_rd: String = l.chars().collect();
        line_rd.trim_left();
        let new_len = line_rd.trim_right().len();
        line_rd.truncate(new_len);
        let un_squished = &line_rd[..];
        let content = un_squished.split_whitespace().collect::<Vec<_>>();
        write!(writer, "{:?}", content);
    }

}
