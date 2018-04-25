use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// A state with transitions
pub struct State {
    pub name: String,
    pub transitions: HashMap<Symbol, Transition>
}

/// A state transition
pub struct Transition {
    pub next_state: usize,
    pub note: Note
}

/// A MIDI note
pub struct Note {
    /// The pitch of the note
    pub pitch: u8,

    /// How loud a note sounds
    pub velocity: u8,

    /// Duration of the note in milliseconds
    pub duration: u8
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
    pub fn from(string: &str) -> Vec<Symbol> {
        string
            .chars()
            .map(Symbol::new)
            .collect()
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