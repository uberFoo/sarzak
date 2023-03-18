use std::{env, fs};

use sarzak::dwarf::parse;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let ast = parse(&src).expect("Failed to parse file");
    dbg!(ast);
}
