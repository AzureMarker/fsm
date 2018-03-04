use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// A state with transitions
#[derive(Clone)]
pub struct State {
    pub name: String,
    pub transitions: HashMap<Symbol, usize>
}

/// Represents a symbol in an alphabet
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum Symbol {
    Epsilon, Letter(char)
}

impl Symbol {
    pub fn new(letter: char) -> Symbol {
        Symbol::Letter(letter)
    }

    /// Create symbols from a string
    pub fn from<T: Into<String>>(string: T) -> Vec<Symbol> {
        string.into().chars().map(|c| Symbol::new(c)).collect()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Symbol::Epsilon => write!(f, "ϵ"),
            Symbol::Letter(c) => write!(f, "{}", c)
        }
    }
}