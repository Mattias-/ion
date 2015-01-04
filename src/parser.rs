use std::borrow::ToOwned;
use std::ops::Deref;
use regex::{Regex, Captures};

use abs::Expr;
use abs::Expr::{Id, LitInt, Neg, Plus, Minus};
use abs::Stm;
use abs::Stm::{Vardef, Assign};
use abs::Type;

#[deriving(Show)]
pub struct Line<'a>(pub &'a str);
impl<'a> Deref<str> for Line<'a> {
    fn deref<'b>(&'b self) -> &'b str {
        let Line(s) = *self;
        s
    }
}

struct ParseRule {
    name: String,
    regex: Regex,
}
pub struct Parser {
    rules: Vec<ParseRule>
}

impl Parser {

    pub fn new() -> Parser {
        let id = r"([:lower:][:alnum:]*)";
        let typ = r"([:upper:][:alnum:]*)";
        let litint = r"([:digit:]+)";
        let expr = r"(.*)";

        let parse_patterns = vec![
            ("Vardef", vec![id, r" :: ", typ]),
            ("Assign", vec![id, r" = ", expr]),

            ("Type", vec![typ]),

            ("Id", vec![id]),
            ("LitInt", vec![litint]),
            ("Plus", vec![expr, r" \+ ", expr]),
            ("Minus", vec![expr, r" - ", expr]),
            ("Neg", vec![r"-", expr]),
        ];

        Parser {
            rules: parse_patterns
                .into_iter()
                .map(|(name, pattern_parts)|
                     (name, format!("^{}$", { let s: String = pattern_parts.concat(); s })))
                .map(|(name, pattern)|
                     ParseRule {
                         name: name.to_owned(),
                         regex: Regex::new(pattern.as_slice()).unwrap()
                     }
                ).collect(),
        }
    }

    pub fn parse(&self, s: Vec<Line>) -> Vec<Stm> {
        s.into_iter().map(|line| self.parse_stm(&*line)).collect()
    }

    fn parse_stm(&self, s: &str) -> Stm {
        for rt in self.rules.iter() {
            let ref rule = *rt;
            if rule.regex.is_match(s) {
                let c = rule.regex.captures(s).expect("No captures");
                return match rule.name.as_slice() {
                    "Vardef" => self.vardef(c),
                    "Assign" => self.assign(c),
                    _ => panic!("Bad match: {}", rule.name)
                };
            }
        }
        panic!("No match: {}", s);
    }

    fn vardef(&self, cap: Captures) -> Stm {
        let e = self.parse_expr(cap.at(1).unwrap());
        let t = cap.at(2).and_then(|s| s.parse()).unwrap();
        return Vardef(e, Type(t));
    }

    fn assign(&self, cap: Captures) -> Stm {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Assign(e1, e2);
    }

    fn parse_expr(&self, s: &str) -> Expr {
        for rt in self.rules.iter() {
            let ref rule = *rt;
            if rule.regex.is_match(s) {
                let c = rule.regex.captures(s).expect("No captures");
                return match rule.name.as_slice() {
                    "Id" => self.id(c),
                    "LitInt" => self.litint(c),
                    "Neg" => self.neg(c),
                    "Plus" => self.plus(c),
                    "Minus" => self.minus(c),
                    _ => panic!("Bad match: {}", rule.name)
                };
            }
        }
        panic!("No match: {}", s);
    }

    fn id(&self, cap: Captures) -> Expr {
        let s = cap.at(1).and_then(from_str).unwrap();
        return Id(s);
    }

    fn litint(&self, cap: Captures) -> Expr {
        let i = cap.at(1).and_then(from_str).unwrap();
        return LitInt(i);
    }

    fn neg(&self, cap: Captures) -> Expr {
        let e = self.parse_expr(cap.at(1).unwrap());
        return Neg(box e);
    }

    fn plus(&self, cap: Captures) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Plus(box e1, box e2);
    }

    fn minus(&self, cap: Captures) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Minus(box e1, box e2);
    }
}

