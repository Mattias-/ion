
extern crate regex;

use std::io::File;

use parser::{Line, Parser};
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
    for stm in stms.iter() {
        e.exec_stm((*stm).clone());
    }
    e.print_env();
}

fn preprocess<'a>(s: &'a String) -> Vec<Line>{
    s.lines()
     .filter_map(|line| if line == "" { None } else { Some(Line(line)) })
     .collect()
}
