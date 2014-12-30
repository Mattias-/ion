
extern crate regex;

use std::io::File;

use parser::Parser;
use eval::Eval;

mod abs;
mod parser;
mod eval;

fn main() {
    let args = std::os::args();
    if args.len() < 2 {
        panic!("Please provide a file");
    }
    let path = Path::new(&args[1]);
    let s = File::open(&path).read_to_string().unwrap();

    let lines = preprocess(&s);

    let p = Parser::new();
    let stms = p.parse(lines);
    println!("Parsed:\n{}\n", stms);

    let mut e = Eval::new();
    e.exec_stms(stms);
}

fn preprocess<'a>(s: &'a String) -> Vec<&str>{
    let mut res: Vec<&str> = vec![];
    for line in s.as_slice().lines() {
        match line {
            "" => {} // Discard empty lines
            _ => res.push(line)
        }
    }
    return res
}

