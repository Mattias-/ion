
use std::collections::HashMap;

use abs::Expr;
use abs::Expr::{Id, LitInt, Neg, Plus, Minus};
use abs::Stm;
use abs::Stm::{Vardef, Assign};

#[derive(Show)]
struct Env(HashMap<String, int>);

impl Env {

    fn new() -> Env {
        return Env(HashMap::new());
    }

    fn add(&mut self, id: String, value: int) {
        let ref mut m = self.0;
        m.insert(id, value);
    }

    fn lookup(&mut self, id:String) -> int {
        let ref mut m = self.0;
        return *m.get(&id).expect("Undefined variable");
    }
}

pub struct Eval {
    env: Env,
}

impl Eval {

    pub fn new() -> Eval {
        Eval {env: Env::new()}
    }

    pub fn print_env(&self) {
        println!("Environment:\n{}", self.env);
    }

    pub fn exec_stm(&mut self, stm: Stm) {
        match stm {
            Vardef(Id(_), _) => {},
            Assign(Id(s), e) => {
                let x = self.eval(e);
                self.env.add(s, x)
            },
            _ => panic!("Unknown stm: {} in exec", stm)
        };
    }

    fn eval(&mut self, expr: Expr) -> int {
        match expr {
            Id(s) => self.env.lookup(s),
            LitInt(i) => i,
            Neg(box e) => - self.eval(e),
            Plus(box e1, box e2) => self.eval(e1) + self.eval(e2),
            Minus(box e1, box e2) => self.eval(e1) - self.eval(e2),
        }
    }
}
