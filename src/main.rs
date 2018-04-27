#[macro_use] extern crate fsm;

use std::collections::{HashMap, HashSet};
use std::env;
use fsm::common::{Symbol};

fn main() {
    /*
    // Legend of Zelda - Main Theme:
    Gf Ba Ba Bf
    Ad Ab Hd
    Gf Bb Bb Bf
    Ae Ac He
    Df Ea Bf Af Ah Aj Ak
    Gm Bm Bm An Ap
    Gr Br Br Ap An
    Cp An Fm Dm
    Bk Ak Am Fn Bm Bk
    Bi Ai Ak Fm Bk Bi
    Bh Ah Aj Fl Do
    Bm Aa Aa Ba Aa Aa Ba Aa Aa Ba Aa Aa
    Df Ea Bf Af Ah Aj Ak
    Gm Bm Bm An Ap
    Hr Du
    Dt Fq Dm
    Hn Dr
    Dq Fm Dm
    Hn Dr
    Dq Fm Dj
    Hk Dn
    Dm Fi Df
    Bh Ah Aj Fl Do
    Bm Aa Aa Ba Aa Aa
    Ha Ha

    // Compiled program input
    GfBaBaBfAdAbHdGfBbBbBfAeAcHeDfEaBfAfAhAjAkGmBmBmAnApGrBrBrApAnCpAnFmDmBkAkAmFnBmBkBiAiAkFmBkBiBhAhAjFlDoBmAaAaBaAaAaBaAaAaBaAaAaDfEaBfAfAhAjAkGmBmBmAnApHrDuDtFqDmHnDrDqFmDmHnDrDqFmDjHkDnDmFiDfBhAhAjFlDoBmAaAaBaAaAaHaHa
    */
    let mut dfa = dfa!(
        {
            a, b, c, d, e, f, g, h, i, j, k, l, m,
            n, o, p, q, r, s, t, u, v, w, x, y, z,
            A, B, C, D, E, F, G, H, I
        },
        {
            sixteenth, eighth, dot_eighth, quarter, dot_quarter, half, half_eighth, dot_half, dot_dot_half
        },
        {
            (sixteenth, a) -> [sixteenth, 65, 80, 1], (eighth, a) -> [eighth, 65, 80, 2],
            (sixteenth, b) -> [sixteenth, 66, 80, 1], (eighth, b) -> [eighth, 66, 80, 2],
            (sixteenth, c) -> [sixteenth, 67, 80, 1], (eighth, c) -> [eighth, 67, 80, 2],
            (sixteenth, d) -> [sixteenth, 68, 80, 1], (eighth, d) -> [eighth, 68, 80, 2],
            (sixteenth, e) -> [sixteenth, 69, 80, 1], (eighth, e) -> [eighth, 69, 80, 2],
            (sixteenth, f) -> [sixteenth, 70, 80, 1], (eighth, f) -> [eighth, 70, 80, 2],
            (sixteenth, g) -> [sixteenth, 71, 80, 1], (eighth, g) -> [eighth, 71, 80, 2],
            (sixteenth, h) -> [sixteenth, 72, 80, 1], (eighth, h) -> [eighth, 72, 80, 2],
            (sixteenth, i) -> [sixteenth, 73, 80, 1], (eighth, i) -> [eighth, 73, 80, 2],
            (sixteenth, j) -> [sixteenth, 74, 80, 1], (eighth, j) -> [eighth, 74, 80, 2],
            (sixteenth, k) -> [sixteenth, 75, 80, 1], (eighth, k) -> [eighth, 75, 80, 2],
            (sixteenth, l) -> [sixteenth, 76, 80, 1], (eighth, l) -> [eighth, 76, 80, 2],
            (sixteenth, m) -> [sixteenth, 77, 80, 1], (eighth, m) -> [eighth, 77, 80, 2],
            (sixteenth, n) -> [sixteenth, 78, 80, 1], (eighth, n) -> [eighth, 78, 80, 2],
            (sixteenth, o) -> [sixteenth, 79, 80, 1], (eighth, o) -> [eighth, 79, 80, 2],
            (sixteenth, p) -> [sixteenth, 80, 80, 1], (eighth, p) -> [eighth, 80, 80, 2],
            (sixteenth, q) -> [sixteenth, 81, 80, 1], (eighth, q) -> [eighth, 81, 80, 2],
            (sixteenth, r) -> [sixteenth, 82, 80, 1], (eighth, r) -> [eighth, 82, 80, 2],
            (sixteenth, s) -> [sixteenth, 83, 80, 1], (eighth, s) -> [eighth, 83, 80, 2],
            (sixteenth, t) -> [sixteenth, 84, 80, 1], (eighth, t) -> [eighth, 84, 80, 2],
            (sixteenth, u) -> [sixteenth, 85, 80, 1], (eighth, u) -> [eighth, 85, 80, 2],
            (sixteenth, v) -> [sixteenth, 86, 80, 1], (eighth, v) -> [eighth, 86, 80, 2],
            (sixteenth, w) -> [sixteenth, 87, 80, 1], (eighth, w) -> [eighth, 87, 80, 2],
            (sixteenth, x) -> [sixteenth, 88, 80, 1], (eighth, x) -> [eighth, 88, 80, 2],
            (sixteenth, y) -> [sixteenth, 89, 80, 1], (eighth, y) -> [eighth, 89, 80, 2],
            (sixteenth, z) -> [sixteenth, 90, 80, 1], (eighth, z) -> [eighth, 90, 80, 2],
            (sixteenth, A) -> [sixteenth   , 0, 0, 0], (eighth, A) -> [sixteenth   , 0, 0, 0],
            (sixteenth, B) -> [eighth      , 0, 0, 0], (eighth, B) -> [eighth      , 0, 0, 0],
            (sixteenth, C) -> [dot_eighth  , 0, 0, 0], (eighth, C) -> [dot_eighth  , 0, 0, 0],
            (sixteenth, D) -> [quarter     , 0, 0, 0], (eighth, D) -> [quarter     , 0, 0, 0],
            (sixteenth, E) -> [dot_quarter , 0, 0, 0], (eighth, E) -> [dot_quarter , 0, 0, 0],
            (sixteenth, F) -> [half        , 0, 0, 0], (eighth, F) -> [half        , 0, 0, 0],
            (sixteenth, G) -> [half_eighth , 0, 0, 0], (eighth, G) -> [half_eighth , 0, 0, 0],
            (sixteenth, H) -> [dot_half    , 0, 0, 0], (eighth, H) -> [dot_half    , 0, 0, 0],
            (sixteenth, I) -> [dot_dot_half, 0, 0, 0], (eighth, I) -> [dot_dot_half, 0, 0, 0],
            (dot_eighth, a) -> [dot_eighth, 65, 80, 3], (quarter, a) -> [quarter, 65, 80, 4],
            (dot_eighth, b) -> [dot_eighth, 66, 80, 3], (quarter, b) -> [quarter, 66, 80, 4],
            (dot_eighth, c) -> [dot_eighth, 67, 80, 3], (quarter, c) -> [quarter, 67, 80, 4],
            (dot_eighth, d) -> [dot_eighth, 68, 80, 3], (quarter, d) -> [quarter, 68, 80, 4],
            (dot_eighth, e) -> [dot_eighth, 69, 80, 3], (quarter, e) -> [quarter, 69, 80, 4],
            (dot_eighth, f) -> [dot_eighth, 70, 80, 3], (quarter, f) -> [quarter, 70, 80, 4],
            (dot_eighth, g) -> [dot_eighth, 71, 80, 3], (quarter, g) -> [quarter, 71, 80, 4],
            (dot_eighth, h) -> [dot_eighth, 72, 80, 3], (quarter, h) -> [quarter, 72, 80, 4],
            (dot_eighth, i) -> [dot_eighth, 73, 80, 3], (quarter, i) -> [quarter, 73, 80, 4],
            (dot_eighth, j) -> [dot_eighth, 74, 80, 3], (quarter, j) -> [quarter, 74, 80, 4],
            (dot_eighth, k) -> [dot_eighth, 75, 80, 3], (quarter, k) -> [quarter, 75, 80, 4],
            (dot_eighth, l) -> [dot_eighth, 76, 80, 3], (quarter, l) -> [quarter, 76, 80, 4],
            (dot_eighth, m) -> [dot_eighth, 77, 80, 3], (quarter, m) -> [quarter, 77, 80, 4],
            (dot_eighth, n) -> [dot_eighth, 78, 80, 3], (quarter, n) -> [quarter, 78, 80, 4],
            (dot_eighth, o) -> [dot_eighth, 79, 80, 3], (quarter, o) -> [quarter, 79, 80, 4],
            (dot_eighth, p) -> [dot_eighth, 80, 80, 3], (quarter, p) -> [quarter, 80, 80, 4],
            (dot_eighth, q) -> [dot_eighth, 81, 80, 3], (quarter, q) -> [quarter, 81, 80, 4],
            (dot_eighth, r) -> [dot_eighth, 82, 80, 3], (quarter, r) -> [quarter, 82, 80, 4],
            (dot_eighth, s) -> [dot_eighth, 83, 80, 3], (quarter, s) -> [quarter, 83, 80, 4],
            (dot_eighth, t) -> [dot_eighth, 84, 80, 3], (quarter, t) -> [quarter, 84, 80, 4],
            (dot_eighth, u) -> [dot_eighth, 85, 80, 3], (quarter, u) -> [quarter, 85, 80, 4],
            (dot_eighth, v) -> [dot_eighth, 86, 80, 3], (quarter, v) -> [quarter, 86, 80, 4],
            (dot_eighth, w) -> [dot_eighth, 87, 80, 3], (quarter, w) -> [quarter, 87, 80, 4],
            (dot_eighth, x) -> [dot_eighth, 88, 80, 3], (quarter, x) -> [quarter, 88, 80, 4],
            (dot_eighth, y) -> [dot_eighth, 89, 80, 3], (quarter, y) -> [quarter, 89, 80, 4],
            (dot_eighth, z) -> [dot_eighth, 90, 80, 3], (quarter, z) -> [quarter, 90, 80, 4],
            (dot_eighth, A) -> [sixteenth   , 0, 0, 0], (quarter, A) -> [sixteenth   , 0, 0, 0],
            (dot_eighth, B) -> [eighth      , 0, 0, 0], (quarter, B) -> [eighth      , 0, 0, 0],
            (dot_eighth, C) -> [dot_eighth  , 0, 0, 0], (quarter, C) -> [dot_eighth  , 0, 0, 0],
            (dot_eighth, D) -> [quarter     , 0, 0, 0], (quarter, D) -> [quarter     , 0, 0, 0],
            (dot_eighth, E) -> [dot_quarter , 0, 0, 0], (quarter, E) -> [dot_quarter , 0, 0, 0],
            (dot_eighth, F) -> [half        , 0, 0, 0], (quarter, F) -> [half        , 0, 0, 0],
            (dot_eighth, G) -> [half_eighth , 0, 0, 0], (quarter, G) -> [half_eighth , 0, 0, 0],
            (dot_eighth, H) -> [dot_half    , 0, 0, 0], (quarter, H) -> [dot_half    , 0, 0, 0],
            (dot_eighth, I) -> [dot_dot_half, 0, 0, 0], (quarter, I) -> [dot_dot_half, 0, 0, 0],
            (dot_quarter, a) -> [dot_quarter, 65, 80, 6], (half, a) -> [half, 65, 80, 8],
            (dot_quarter, b) -> [dot_quarter, 66, 80, 6], (half, b) -> [half, 66, 80, 8],
            (dot_quarter, c) -> [dot_quarter, 67, 80, 6], (half, c) -> [half, 67, 80, 8],
            (dot_quarter, d) -> [dot_quarter, 68, 80, 6], (half, d) -> [half, 68, 80, 8],
            (dot_quarter, e) -> [dot_quarter, 69, 80, 6], (half, e) -> [half, 69, 80, 8],
            (dot_quarter, f) -> [dot_quarter, 70, 80, 6], (half, f) -> [half, 70, 80, 8],
            (dot_quarter, g) -> [dot_quarter, 71, 80, 6], (half, g) -> [half, 71, 80, 8],
            (dot_quarter, h) -> [dot_quarter, 72, 80, 6], (half, h) -> [half, 72, 80, 8],
            (dot_quarter, i) -> [dot_quarter, 73, 80, 6], (half, i) -> [half, 73, 80, 8],
            (dot_quarter, j) -> [dot_quarter, 74, 80, 6], (half, j) -> [half, 74, 80, 8],
            (dot_quarter, k) -> [dot_quarter, 75, 80, 6], (half, k) -> [half, 75, 80, 8],
            (dot_quarter, l) -> [dot_quarter, 76, 80, 6], (half, l) -> [half, 76, 80, 8],
            (dot_quarter, m) -> [dot_quarter, 77, 80, 6], (half, m) -> [half, 77, 80, 8],
            (dot_quarter, n) -> [dot_quarter, 78, 80, 6], (half, n) -> [half, 78, 80, 8],
            (dot_quarter, o) -> [dot_quarter, 79, 80, 6], (half, o) -> [half, 79, 80, 8],
            (dot_quarter, p) -> [dot_quarter, 80, 80, 6], (half, p) -> [half, 80, 80, 8],
            (dot_quarter, q) -> [dot_quarter, 81, 80, 6], (half, q) -> [half, 81, 80, 8],
            (dot_quarter, r) -> [dot_quarter, 82, 80, 6], (half, r) -> [half, 82, 80, 8],
            (dot_quarter, s) -> [dot_quarter, 83, 80, 6], (half, s) -> [half, 83, 80, 8],
            (dot_quarter, t) -> [dot_quarter, 84, 80, 6], (half, t) -> [half, 84, 80, 8],
            (dot_quarter, u) -> [dot_quarter, 85, 80, 6], (half, u) -> [half, 85, 80, 8],
            (dot_quarter, v) -> [dot_quarter, 86, 80, 6], (half, v) -> [half, 86, 80, 8],
            (dot_quarter, w) -> [dot_quarter, 87, 80, 6], (half, w) -> [half, 87, 80, 8],
            (dot_quarter, x) -> [dot_quarter, 88, 80, 6], (half, x) -> [half, 88, 80, 8],
            (dot_quarter, y) -> [dot_quarter, 89, 80, 6], (half, y) -> [half, 89, 80, 8],
            (dot_quarter, z) -> [dot_quarter, 90, 80, 6], (half, z) -> [half, 90, 80, 8],
            (dot_quarter, A) -> [sixteenth   , 0, 0, 0], (half, A) -> [sixteenth   , 0, 0, 0],
            (dot_quarter, B) -> [eighth      , 0, 0, 0], (half, B) -> [eighth      , 0, 0, 0],
            (dot_quarter, C) -> [dot_eighth  , 0, 0, 0], (half, C) -> [dot_eighth  , 0, 0, 0],
            (dot_quarter, D) -> [quarter     , 0, 0, 0], (half, D) -> [quarter     , 0, 0, 0],
            (dot_quarter, E) -> [dot_quarter , 0, 0, 0], (half, E) -> [dot_quarter , 0, 0, 0],
            (dot_quarter, F) -> [half        , 0, 0, 0], (half, F) -> [half        , 0, 0, 0],
            (dot_quarter, G) -> [half_eighth , 0, 0, 0], (half, G) -> [half_eighth , 0, 0, 0],
            (dot_quarter, H) -> [dot_half    , 0, 0, 0], (half, H) -> [dot_half    , 0, 0, 0],
            (dot_quarter, I) -> [dot_dot_half, 0, 0, 0], (half, I) -> [dot_dot_half, 0, 0, 0],
            (half_eighth, a) -> [half, 65, 80, 10],       (dot_half, a) -> [dot_half, 65, 80, 12],
            (half_eighth, b) -> [half, 66, 80, 10],       (dot_half, b) -> [dot_half, 66, 80, 12],
            (half_eighth, c) -> [half, 67, 80, 10],       (dot_half, c) -> [dot_half, 67, 80, 12],
            (half_eighth, d) -> [half, 68, 80, 10],       (dot_half, d) -> [dot_half, 68, 80, 12],
            (half_eighth, e) -> [half, 69, 80, 10],       (dot_half, e) -> [dot_half, 69, 80, 12],
            (half_eighth, f) -> [half, 70, 80, 10],       (dot_half, f) -> [dot_half, 70, 80, 12],
            (half_eighth, g) -> [half, 71, 80, 10],       (dot_half, g) -> [dot_half, 71, 80, 12],
            (half_eighth, h) -> [half, 72, 80, 10],       (dot_half, h) -> [dot_half, 72, 80, 12],
            (half_eighth, i) -> [half, 73, 80, 10],       (dot_half, i) -> [dot_half, 73, 80, 12],
            (half_eighth, j) -> [half, 74, 80, 10],       (dot_half, j) -> [dot_half, 74, 80, 12],
            (half_eighth, k) -> [half, 75, 80, 10],       (dot_half, k) -> [dot_half, 75, 80, 12],
            (half_eighth, l) -> [half, 76, 80, 10],       (dot_half, l) -> [dot_half, 76, 80, 12],
            (half_eighth, m) -> [half, 77, 80, 10],       (dot_half, m) -> [dot_half, 77, 80, 12],
            (half_eighth, n) -> [half, 78, 80, 10],       (dot_half, n) -> [dot_half, 78, 80, 12],
            (half_eighth, o) -> [half, 79, 80, 10],       (dot_half, o) -> [dot_half, 79, 80, 12],
            (half_eighth, p) -> [half, 80, 80, 10],       (dot_half, p) -> [dot_half, 80, 80, 12],
            (half_eighth, q) -> [half, 81, 80, 10],       (dot_half, q) -> [dot_half, 81, 80, 12],
            (half_eighth, r) -> [half, 82, 80, 10],       (dot_half, r) -> [dot_half, 82, 80, 12],
            (half_eighth, s) -> [half, 83, 80, 10],       (dot_half, s) -> [dot_half, 83, 80, 12],
            (half_eighth, t) -> [half, 84, 80, 10],       (dot_half, t) -> [dot_half, 84, 80, 12],
            (half_eighth, u) -> [half, 85, 80, 10],       (dot_half, u) -> [dot_half, 85, 80, 12],
            (half_eighth, v) -> [half, 86, 80, 10],       (dot_half, v) -> [dot_half, 86, 80, 12],
            (half_eighth, w) -> [half, 87, 80, 10],       (dot_half, w) -> [dot_half, 87, 80, 12],
            (half_eighth, x) -> [half, 88, 80, 10],       (dot_half, x) -> [dot_half, 88, 80, 12],
            (half_eighth, y) -> [half, 89, 80, 10],       (dot_half, y) -> [dot_half, 89, 80, 12],
            (half_eighth, z) -> [half, 90, 80, 10],       (dot_half, z) -> [dot_half, 90, 80, 12],
            (half_eighth, A) -> [sixteenth   , 0, 0, 0], (dot_half, A) -> [sixteenth   , 0, 0, 0],
            (half_eighth, B) -> [eighth      , 0, 0, 0], (dot_half, B) -> [eighth      , 0, 0, 0],
            (half_eighth, C) -> [dot_eighth  , 0, 0, 0], (dot_half, C) -> [dot_eighth  , 0, 0, 0],
            (half_eighth, D) -> [quarter     , 0, 0, 0], (dot_half, D) -> [quarter     , 0, 0, 0],
            (half_eighth, E) -> [dot_quarter , 0, 0, 0], (dot_half, E) -> [dot_quarter , 0, 0, 0],
            (half_eighth, F) -> [half        , 0, 0, 0], (dot_half, F) -> [half        , 0, 0, 0],
            (half_eighth, G) -> [half_eighth , 0, 0, 0], (dot_half, G) -> [half_eighth , 0, 0, 0],
            (half_eighth, H) -> [dot_half    , 0, 0, 0], (dot_half, H) -> [dot_half    , 0, 0, 0],
            (half_eighth, I) -> [dot_dot_half, 0, 0, 0], (dot_half, I) -> [dot_dot_half, 0, 0, 0],
            (dot_dot_half, a) -> [dot_dot_half, 65, 80, 14],
            (dot_dot_half, b) -> [dot_dot_half, 66, 80, 14],
            (dot_dot_half, c) -> [dot_dot_half, 67, 80, 14],
            (dot_dot_half, d) -> [dot_dot_half, 68, 80, 14],
            (dot_dot_half, e) -> [dot_dot_half, 69, 80, 14],
            (dot_dot_half, f) -> [dot_dot_half, 70, 80, 14],
            (dot_dot_half, g) -> [dot_dot_half, 71, 80, 14],
            (dot_dot_half, h) -> [dot_dot_half, 72, 80, 14],
            (dot_dot_half, i) -> [dot_dot_half, 73, 80, 14],
            (dot_dot_half, j) -> [dot_dot_half, 74, 80, 14],
            (dot_dot_half, k) -> [dot_dot_half, 75, 80, 14],
            (dot_dot_half, l) -> [dot_dot_half, 76, 80, 14],
            (dot_dot_half, m) -> [dot_dot_half, 77, 80, 14],
            (dot_dot_half, n) -> [dot_dot_half, 78, 80, 14],
            (dot_dot_half, o) -> [dot_dot_half, 79, 80, 14],
            (dot_dot_half, p) -> [dot_dot_half, 80, 80, 14],
            (dot_dot_half, q) -> [dot_dot_half, 81, 80, 14],
            (dot_dot_half, r) -> [dot_dot_half, 82, 80, 14],
            (dot_dot_half, s) -> [dot_dot_half, 83, 80, 14],
            (dot_dot_half, t) -> [dot_dot_half, 84, 80, 14],
            (dot_dot_half, u) -> [dot_dot_half, 85, 80, 14],
            (dot_dot_half, v) -> [dot_dot_half, 86, 80, 14],
            (dot_dot_half, w) -> [dot_dot_half, 87, 80, 14],
            (dot_dot_half, x) -> [dot_dot_half, 88, 80, 14],
            (dot_dot_half, y) -> [dot_dot_half, 89, 80, 14],
            (dot_dot_half, z) -> [dot_dot_half, 90, 80, 14],
            (dot_dot_half, A) -> [sixteenth   , 0, 0, 0],
            (dot_dot_half, B) -> [eighth      , 0, 0, 0],
            (dot_dot_half, C) -> [dot_eighth  , 0, 0, 0],
            (dot_dot_half, D) -> [quarter     , 0, 0, 0],
            (dot_dot_half, E) -> [dot_quarter , 0, 0, 0],
            (dot_dot_half, F) -> [half        , 0, 0, 0],
            (dot_dot_half, G) -> [half_eighth , 0, 0, 0],
            (dot_dot_half, H) -> [dot_half    , 0, 0, 0],
            (dot_dot_half, I) -> [dot_dot_half, 0, 0, 0]
        },
        sixteenth,
        {
            sixteenth, eighth, dot_eighth, quarter, dot_quarter, half, half_eighth, dot_half, dot_dot_half
        }
    );

    println!("{}", dfa);

    let input = env::args().nth(1);

    if let Some(string) = input {
        println!("Input: {}", string);
        println!("Result: {}", dfa.accepts(Symbol::from(&string)));
    } else {
        println!("Usage: {} [string]", env::args().nth(0).unwrap())
    }
}
