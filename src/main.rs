#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(core)]
#![feature(io)]
#![feature(path)]
#![feature(env)]
#![feature(collections)]

extern crate regex;

use std::old_io::File;

use parser::{Line, Parser};
use eval::Eval;

mod abs;
mod parser;
mod eval;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = Path::new(args.next().unwrap());
    let s = File::open(&path).read_to_string().unwrap();

    let p;
    let lines = preprocess(s.as_slice());
    p = Parser::new();
    let stms = p.parse(lines);
    println!("Parsed:\n{:?}\n", stms);

    let mut e = Eval::new();
    for stm in stms.iter() {
        e.exec_stm((*stm).clone());
    }
    e.print_env();
}

fn preprocess(s: &str) -> Vec<Line>{
    fn f(x: &str) -> Option<Line>{
        if x == "" {
            None
        } else {
            Some(Line{content: x})
        }
    }
    s.lines_any().filter_map(f).collect()
}
