//! Deterministic Finite Automata (DFA)

use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use common::{State, Symbol};

#[macro_export]
macro_rules! dfa {
    (
        {$($letter: ident),+},
        {$($state: ident),+},
        {$(($delta_state: ident, $delta_letter: ident) -> $delta_result: expr),*},
        $q0: ident,
        {$($accepting: ident),*}
    ) => {{
        $(
            let $letter = Symbol::new(stringify!($letter).chars().next().unwrap());
        )*

        let state_names = vec![
            $(
                stringify!($state),
            )*
        ];

        $(
            let mut $state: HashMap<Symbol, usize> = HashMap::new();
        )*

        $(
            $delta_state.insert(
                $delta_letter,
                state_names.iter().position(|s| *s == stringify!($delta_result)).unwrap()
            );
        )*

        let states = vec![$(
            State { name: stringify!($state).to_owned(), transitions: $state },
        )*];

        let mut alphabet = HashSet::new();

        $(
            alphabet.insert($letter);
        )*

        let start_state = {
            state_names.iter().position(|s| *s == stringify!($q0)).unwrap()
        };

        let accepting_states = vec![
            $(
                state_names.iter().position(|s| *s == stringify!($accepting)).unwrap(),
            )*
        ];

        DFA::new(alphabet, states, start_state, accepting_states).unwrap()
    }}
}

pub struct DFA {
    alphabet: HashSet<Symbol>,
    states: Vec<State>,
    start_state: usize,
    accepting_states: Vec<usize>,
    current_state_index: usize
}

impl DFA {
    pub fn new(
        alphabet: HashSet<Symbol>,
        states: Vec<State>,
        start_state: usize,
        accepting_states: Vec<usize>,
    ) -> Result<DFA, String> {
        // Verify all transitions are present
        for state in states.clone() {
            for symbol in alphabet.clone() {
                if !state.transitions.contains_key(&symbol) {
                    return Err("Invalid DFA: Missing transition".to_owned());
                }
            }
        }

        Ok(
            DFA {
                alphabet,
                states,
                start_state,
                accepting_states,
                current_state_index: start_state
            }
        )
    }

    pub fn get_current_state(&self) -> &State {
        &self.states[self.current_state_index]
    }

    pub fn accepts(&mut self, string: Vec<Symbol>) -> bool {
        for symbol in string {
            self.step(symbol);
        }

        self.is_accepting()
    }

    pub fn step(&mut self, symbol: Symbol) {
        let state = self.states.get(self.current_state_index).unwrap();
        self.current_state_index = *state.transitions.get(&symbol).unwrap();
        let new_state = self.states.get(self.current_state_index).unwrap();
        println!("δ({}, {}) => {}", state.name, symbol, new_state.name);
    }

    pub fn is_accepting(&self) -> bool {
        self.accepting_states.contains(&self.current_state_index)
    }
}

impl Display for DFA {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "DFA(Σ, Q, δ, q0, F): {{")?;
        writeln!(f, "  Σ: {{")?;

        for symbol in self.alphabet.iter() {
            writeln!(f, "    {},", symbol)?;
        }

        writeln!(f, "  }},")?;
        writeln!(f, "  Q: {{")?;

        for state in self.states.iter() {
            writeln!(f, "    {},", state.name)?;
        }

        writeln!(f, "  }},")?;
        writeln!(f, "  δ: {{")?;

        for state in self.states.iter() {
            for (symbol, state_index) in state.transitions.iter() {
                let new_state = &self.states[*state_index];

                writeln!(f, "    δ({}, {}) -> {},", state.name, symbol, new_state.name)?;
            }
        }

        writeln!(f, "  }},")?;
        writeln!(f, "  q0: {},", self.states[self.start_state].name)?;
        writeln!(f, "  F: {{")?;

        for state_index in self.accepting_states.iter() {
            let state = &self.states[*state_index];

            writeln!(f, "    {},", state.name)?;
        }

        writeln!(f, "  }}\n)")?;

        Ok(())
    }
}