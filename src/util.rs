//!Utilities Module
//!Various Helper functions and error definitions used throughout the project live here
use gen;
use http::LinesFeeder;
use colored::*;

use std::io;
use std::ops::Deref;


pub fn read_y_n() -> bool {
    loop {
        println!("         {} {} or {} {}", "??".clear(), "Y".bright_green(), "N".bright_red(), "??".clear());
        let i = read_in_ln();
        match i {
            Some(s) => {
                match s.trim() {
                    "Y" | "y" => {
                        //println!("read \"Y\" or \"y\". Confirmed!");
                        return true;
                    }
                    "N" | "n" => {
                        //println!("read \"N\" or \"n\". Canceled!");
                        return false;
                    }
                    _ => {
                        println!("Try again... please, enter a single character: Y, y, N, or n");
                        continue;
                    }
                }
            }
            None => {
                println!("lets try that again, shall we?");
                continue;
            }
        }
    }
}

pub fn read_in_ln() -> Option<String> {
    let mut out = String::new();
    match io::stdin().read_line(&mut out) {
        Ok(..) => {
            if !out.trim().is_empty() {
                if out == "\n".to_string() {
                    println!("Nothing entered.");
                    return None;
                } else if out == "\r".to_string() {
                    println!("Nothing entered.");
                    return None;
                } else {
                    return Some(out)
                }
            } else {
                return None; 
            }
        }
        Err(error) => {
            println!("error reading input: {}", error);
            return None;
        }
    }
}

pub fn read_int() -> i32 {
    loop {
        println!("Enter a whole number...");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(..) => {
                match user_input.trim().parse::<i32>() {
                    Ok(number) => return number,
                    Err(..) => {
                        println!(
                            "that wasn't a number!\nYou entered: \"{}\"",
                            user_input.trim()
                        );
                        continue;
                    }
                }
            }
            Err(error) => {
                println!("error reading input: {}", error);
                println!("let's try again");
                continue;
            }
        };
    }
}
pub fn poem_prompt(feeder: LinesFeeder) {
    println!("Do you want to pause and curate a poem?");
    if read_y_n() {
        println!("Sweet! let\'s find an author!\n");
        //in order to access our persistent storage, within its structure Arc<Mutex<Vec<String>
        //we first need to acquire a "lock", which confirms to us no other thread can attempt
        //to access the underlying data. The error returns a "view" of the data anyway as a
        //fail-safe and because we only need to read what's in the store, that's fine.
        let lock = match feeder.queue.lock() {
            Ok(vec) => vec,
            Err(e) => e.into_inner(),
        };
        //Now that we've acquired our MutexGuard from matcing against .lock(),
        //we have an Arc<Vec<String>
        //it helps to think about the data you need as trapped in a room puzzle.
        //It needs to move from the inside-out! Mutex-->MutexGuard-->Arc-->Deref-->Usable Data
        //This level of integrity is most definitely overkill right now, but it will help us
        //immensely if we end up including any online/multiplayer features requiring server-related
        //functionality. Deref is a trait implemented for the Arc type, included in rust's "ops"
        //package, that converts an Arc<T> into T while maintaining an active count in memory of
        //accessors of T (via the deref trait), ARC meaning Atomic Reference Counter
        let vec = lock.deref().clone();
        //after dereferencing our Arc, all we have left to do is...
        //give the Vec<String> to our seed/generate function.
        //We need to clone the dereferenced Arc<String<Vec>> to avoid it getting wrapped
        //again in the next function. this prevents it
        //from being a reference, which gen::seed_and_generate() won't accept
        //but first--we need to make sure it isn't empty!
        if !&vec.is_empty() {
            gen::seed_and_generate(vec);
        } else {
            println!("Yeah, you should probably read more...");
        }
    } else {
        println!("I didn't want to make a stupid poem anyways...");
    }

}

//ALL THE STUFF BELOW HERE IS DEPRECATED OR ON ITS WAY OUT. DON'T PAY ATTENTION TO IT

// static LOC_SEED_DIR: &'static str = "shakespeare.txt";

// #[derive(Serialize, Deserialize)]
// pub struct PoetryAPIResp {
//     title: String,
//     author: String,
//     lines: Poem,
// }
// #[derive(Serialize, Deserialize)]
// pub struct Poem {
//     linecount: i32,
//     lines: Vec<String>,
// }

// pub fn format_txt() {
//     let p = PathBuf::from(&LOC_SEED_DIR);
//     let f = File::open(&p).unwrap();
//     let file = BufReader::new(&f);
//     let mut writer = BufWriter::new(io::stdout());
//     for (num, line) in file.lines().enumerate() {
//         let l = line.unwrap();
//         let mut b = [0; 4];
//         let mut d = [0, 4];
//         let mut chars: String = l.chars().collect();
//         for c in chars.clone().chars().into_iter() {
//             let p: char = '.';
//             let r: char = '\n';
//             match &c {
//                 p => {
//                     if c == *p {
//                         let index = chars.find('.').unwrap_or_default();
//                         chars.insert(index, '\n' as char);
//                     } else {
//                         continue;
//                     }
//                 }
//                 r => {
//                     let index = chars.find('\n').unwrap_or_default();
//                     chars.remove(index);
//                 }
//                 _ => (),
//             }
//         }
//         writeln!(writer, "{}", chars).unwrap();
//         // if num % 4 == 1 {
//         //     writeln!(writer, "{}", l).unwrap();
//         // }
//     }
// }


// pub fn read_file() {
//     let path = PathBuf::from(&LOC_SEED_DIR);
//     let txt_src = File::open(&path).unwrap();
//     let txt_dest =
//         File::create("output.txt").expect("Couldn't create destination file for output!");
//     let reader = BufReader::new(&txt_src);
//     let mut writer = BufWriter::new(&txt_dest);
//     for (num, line) in reader.lines().enumerate() {
//         let l = line.unwrap();
//         let mut line_rd: String = l.chars().collect();
//         line_rd.trim_left();
//         let new_len = line_rd.trim_right().len();
//         line_rd.truncate(new_len);
//         let un_squished = &line_rd[..];
//         let content = un_squished.split_whitespace().collect::<Vec<_>>();
//         write!(writer, "{:?}", content);
//     }

// }
