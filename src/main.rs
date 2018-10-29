//! Calculate nullable first follow from a file
// This file is part of "nff", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.
use nff::{open_file, NonTerminals};

fn main() {
    // Open file and read as a string
    let grammar = open_file(::std::env::args());

    let start = ::std::time::Instant::now();
    let mut bab = NonTerminals::init(grammar);
    bab.calculate_null_set();
    bab.calculate_first_set();
    bab.calculate_follow_set();
    let end = ::std::time::Instant::now();
    println!("Execution time: {:?}", end - start);
    bab.print_results();
}
