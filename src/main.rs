extern crate regex;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use parser::{Line, Parser};
use eval::Eval;

mod abs;
mod parser;
mod eval;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().unwrap();
    let path = Path::new(&path);
    let mut s = String::new();
    let _ = File::open(&path).expect("couldn't open file").read_to_string(&mut s).unwrap();

    let p;
    let lines = preprocess(&s[..]);
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
    s.lines().filter_map(f).collect()
}
