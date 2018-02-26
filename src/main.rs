extern crate fsm;

use std::collections::{HashMap, HashSet};
use std::env;
use fsm::common::{State, Symbol};
use fsm::dfa::DFA;

fn main() {
    /*
        b
       ^ v     /-\
     -> 0 -a-> 1  a
        ^-_b__/ ^/

     (1, a) => 1
     (1, b) => 0
     (2, a) => 0
     (2, b) => 1
     */

    let symbol1 = Symbol::new('a');
    let symbol2 = Symbol::new('b');

    let mut state1_transitions = HashMap::new();
    let mut state2_transitions = HashMap::new();
    state1_transitions.insert(symbol1, 1);
    state1_transitions.insert( symbol2, 0);
    state2_transitions.insert(symbol1, 1);
    state2_transitions.insert(symbol2, 0);

    let state1 = State { name: "0".to_owned(), transitions: state1_transitions };
    let state2 = State { name: "1".to_owned(), transitions: state2_transitions };
    let states = vec![state1, state2];

    let mut alphabet = HashSet::new();
    alphabet.insert(symbol1);
    alphabet.insert(symbol2);

    let mut dfa = DFA::new(alphabet, states, 0, vec![1]).unwrap();

    println!("{}", dfa);

    let input = env::args().nth(1);

    if let Some(string) = input {
        println!("Input: {}", string);
        println!("Result: {}", dfa.accepts(Symbol::from(string)));
    } else {
        println!("Usage: {} [string]", env::args().nth(0).unwrap())
    }
}
