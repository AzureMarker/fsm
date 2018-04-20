use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use common::{State, Symbol};

/// Create a DFA according to the formal definition
#[macro_export]
macro_rules! dfa {
    (
        {$($letter: ident),+},
        {$($state: ident),+},
        {
            $(($delta_state: ident, $delta_letter: ident) ->
            [$delta_result: expr, $pitch: expr, $velocity: expr, $duration: expr]),*
        },
        $q0: ident,
        {$($accepting: ident),*}
    ) => {{
        // Create the symbols
        $(
            let $letter = $crate::common::Symbol::new(
                stringify!($letter)
                    .chars()
                    .next()
                    .unwrap()
            );
        )*

        // Create a list of state names so we know the index of each state
        let state_names = vec![
            $(
                stringify!($state),
            )*
        ];

        // Initialize the transition maps for each state
        $(
            let mut $state = HashMap::new();
        )*

        // For each state transition, insert a new transition into the state
        $(
            $delta_state.insert(
                $delta_letter,
                $crate::common::Transition {
                    // Use state_names to find the index of the state
                    next_state: state_names
                                    .iter()
                                    .position(|s| *s == stringify!($delta_result))
                                    .unwrap(),
                    // Create the note data
                    note: $crate::common::Note {
                        pitch: $pitch,
                        velocity: $velocity,
                        duration: $duration
                    }
                }
            );
        )*

        // Create the actual list of states
        let states = vec![$(
            $crate::common::State {
                name: stringify!($state).to_owned(),
                transitions: $state
            },
        )*];

        // Create the alphabet from the symbols
        let mut alphabet = HashSet::new();

        $(
            alphabet.insert($letter);
        )*

        // Find the start state index
        let start_state = state_names
            .iter()
            .position(|s| *s == stringify!($q0))
            .unwrap();

        // Find the accepting states' indices
        let accepting_states = vec![
            $(
                state_names
                    .iter()
                    .position(|s| *s == stringify!($accepting))
                    .unwrap(),
            )*
        ];

        // Create the DFA
        $crate::dfa::DFA::new(
            alphabet,
            states,
            start_state,
            accepting_states
        ).unwrap()
    }}
}

/// A Deterministic Finite Automaton (DFA)
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
        for state in &states {
            for symbol in &alphabet {
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

    /// Get the current state of the DFA
    pub fn get_current_state(&self) -> &State {
        &self.states[self.current_state_index]
    }

    /// Run the DFA over a string to see if it accepts it
    pub fn accepts(&mut self, string: Vec<Symbol>) -> bool {
        for symbol in string {
            self.step(symbol);
        }

        self.is_accepting()
    }

    /// Take one step with the given symbol
    pub fn step(&mut self, symbol: Symbol) {
        let state = self.states.get(self.current_state_index).unwrap();
        self.current_state_index = state.transitions.get(&symbol).unwrap().next_state;
        let new_state = self.states.get(self.current_state_index).unwrap();
        println!("δ({}, {}) => {}", state.name, symbol, new_state.name);
    }

    /// Check if the current state is accepting
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
            for (symbol, transition) in state.transitions.iter() {
                let new_state = &self.states[transition.next_state];

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