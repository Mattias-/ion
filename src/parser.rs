
use regex::{Regex, Captures};

use abs::Expr;
use abs::Expr::{Id, Litint, Plus, Minus};
use abs::Stm;
use abs::Stm::{Vardef, Assign};
use abs::Type;

struct SynRule {
    name: String,
    regex: Regex,
}

fn syn_rules<'a>() -> Vec<SynRule> {
    let id = r"([:lower:][:alnum:]*)";
    let typ = r"([:upper:][:alnum:]*)";
    let litint = r"([:digit:]+)";
    let expr = r"(.*)";

    let token_patterns = vec![
        ("Vardef", vec![id, r" :: ", typ]),
        ("Assign", vec![id, r" = ", expr]),

        ("Type", vec![typ]),

        ("Id", vec![id]),
        ("Litint", vec![litint]),
        ("Plus", vec![expr, r" \+ ", expr]),
        ("Minus", vec![expr, r" - ", expr]),
    ];

    let mut rules = vec![];
    for tp in token_patterns.iter() {
        let (name, ref pattern_partials) = *tp;
        let mut sp = String::new();
        sp.push_str("^");
        for pp in pattern_partials.iter() {
            sp.push_str(*pp);
        }
        sp.push_str("$");
        let regex = Regex::new(sp.as_slice()).unwrap();
        rules.push(SynRule {name: String::from_str(name), regex: regex});
    }
    return rules;
}

pub struct Parser {
    rules: Vec<SynRule>
}

impl Parser {

    pub fn new() -> Parser {
        Parser {rules: syn_rules()}
    }

    pub fn parse(&self, s: Vec<&str>) -> Vec<Stm> {
        let mut res: Vec<Stm> = vec![];
        for line in s.iter() {
            let l = self.parse_stm(*line);
            res.push(l);
        }
        return res;
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
        let t = Type(cap.at(2).and_then(from_str).unwrap());
        return Vardef(e, t);
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
                    "Litint" => self.litint(c),
                    "Plus" => self.plus(c),
                    "Minus" => self.minus(c),
                    _ => panic!("Bad match: {}", rule.name)
                };
            }
        }
        panic!("No match: {}", s);
    }

    fn id(&self, cap: Captures) -> Expr {
        return Id(cap.at(1).and_then(from_str).unwrap());
    }

    fn litint(&self, cap: Captures) -> Expr {
        return Litint(cap.at(1).and_then(from_str).unwrap());
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

