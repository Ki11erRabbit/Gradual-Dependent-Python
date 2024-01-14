mod inference;

use std::collections::HashMap;
use rustpython_ast::bigint::BigInt;

pub enum Term {
    Known(KnownTerm),
    Identifier(String),
    Unknown,
}


impl Term {
    pub fn map<O>(self, f: impl FnOnce(self) -> O) -> O {
        f(self)
    }
}


pub enum KnownTerm {
    None,
    Integer(Option<BigInt>),
    Float(Option<f64>),
    String(Option<String>),
    Bytes(Option<Vec<u8>>),
    Tuple(Option<Vec<KnownTerm>>),
    Complex { real: Option<f64>, imag: Option<f64> },
    Bool(Option<bool>),
    Function { args: Option<Vec<Term>>, return_type: Option<Box<Term>> },
    Class { name: String, fields: Option<Vec<Term>> },
}


pub struct Environment {
    pub variables: HashMap<String, Term>,
    pub functions: HashMap<String, Term>,
    pub classes: HashMap<String, Term>,
}

impl Environment {
    pub fn add_variable(&mut self, name: &str, term: Term) {
        self.variables.insert(name.to_string(), term);
    }
}