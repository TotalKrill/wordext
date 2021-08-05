use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "wordpipe")]
struct Opt {
    #[structopt(short, long, default_value = "0")]
    /// extracts the nth word of the input
    nth: usize,

    #[structopt(short, long)]
    /// splits the input on lines and then extracts the nth word of each line if any
    repeat_per_line: bool,
}

fn main() {
    let opt = Opt::from_args();
    let stdin = io::stdin();

    let lines = stdin.lock().lines();
    // take the nth word of each line in the input
    if opt.repeat_per_line {
        let mut first = true;
        for line in lines {
            if !first {
                // Add a newline explicitly before the new word
                println!();
            }
            first = false;
            let line = line.expect("Could not read line from standard in");
            let line = line.trim();
            print_word_from_line(opt.nth, &line);
        }
    } else {
        let lines: Vec<String> = lines.into_iter().filter_map(|e| e.ok()).collect();
        //println!("{:?}", lines);
        let mut line = String::new();

        for l in lines {
            line.push_str(&l);
            line.push_str(" ");
        }
        let line = line.trim();
        //println!("{}", line);
        print_word_from_line(opt.nth, &line);
    }
}

fn print_word_from_line(number: usize, line: &str) {
    let words: Vec<&str> = line.split(" ").collect();
    let word = words.get(number);

    if let Some(word) = word {
        print!("{}", word);
    }
}
