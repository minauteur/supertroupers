//!Utilities Module
//!Various Helper functions and error definitions used throughout the project live here
use std::io::{BufReader, BufWriter, Read, Write};
use std::io::BufRead;
use std::fs::File;
use std::io;
use std::path::PathBuf;

static LOC_SEED_DIR: &'static str = "C:\\Users\\Minauteur\\Desktop\\shakespeare.txt";

pub fn format_txt(p: &PathBuf) {
    let f = File::open(&p).unwrap();
    let file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        if num % 4 == 0 {
            let chars: String = l.chars().skip(1).collect();
            writeln!(writer, ">{}", chars).unwrap();
        }
        if num % 4 == 1 {
            writeln!(writer, "{}", l).unwrap();
        }
    }
}
