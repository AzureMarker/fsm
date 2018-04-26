use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use common::{State, Symbol};
use midi_client::MidiClient;

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
            accepting_states,
            "localhost:1337"
        ).unwrap()
    }}
}

/// A Deterministic Finite Automaton (DFA)
pub struct DFA {
    alphabet: HashSet<Symbol>,
    states: Vec<State>,
    start_state: usize,
    accepting_states: Vec<usize>,
    current_state_index: usize,
    song_time: usize,
    midi_client: MidiClient
}

impl DFA {
    pub fn new(
        alphabet: HashSet<Symbol>,
        states: Vec<State>,
        start_state: usize,
        accepting_states: Vec<usize>,
        midi_server_address: &str
    ) -> Result<DFA, String> {
        // Verify all transitions are present
        for state in &states {
            for symbol in &alphabet {
                if !state.transitions.contains_key(&symbol) {
                    return Err(format!("Invalid DFA: Missing transition ({}, {})", state.name, symbol));
                }
            }
        }

        Ok(
            DFA {
                alphabet,
                states,
                start_state,
                accepting_states,
                current_state_index: start_state,
                song_time: 0,
                midi_client: MidiClient::new(midi_server_address)
                    .map_err(|e| e.description().to_owned())?
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

        // Send EOF to MIDI server
        self.midi_client.close().unwrap();

        self.is_accepting()
    }

    /// Take one step with the given symbol
    pub fn step(&mut self, symbol: Symbol) {
        // Get the current state and transition
        let state: &State = self.states.get(self.current_state_index).unwrap();
        let transition = state.transitions.get(&symbol).unwrap();

        // Apply the transition
        self.current_state_index = transition.next_state;

        // Send the note to be added to the midi song
        self.midi_client.send(&transition.note, self.song_time as u8).unwrap();
        self.song_time += transition.note.duration as usize;

        // Print the transition
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