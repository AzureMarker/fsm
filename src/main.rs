#[macro_use] extern crate fsm;

use std::collections::{HashMap, HashSet};
use std::env;
use fsm::common::{Symbol};

fn main() {
    let mut dfa = dfa!(
        {
            a, b
        },
        {
            s0, s1
        },
        {
            (s0, b) -> [s0, 0, 0, 0],
            (s0, a) -> [s1, 0, 0, 0],
            (s1, b) -> [s0, 0, 0, 0],
            (s1, a) -> [s1, 0, 0, 0]
        },
        s0,
        {
            s1
        }
    );

    println!("{}", dfa);

    let input = env::args().nth(1);

    if let Some(string) = input {
        println!("Input: {}", string);
        println!("Result: {}", dfa.accepts(Symbol::from(string)));
    } else {
        println!("Usage: {} [string]", env::args().nth(0).unwrap())
    }
}
