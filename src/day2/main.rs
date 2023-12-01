use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::env;
use std::{char, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let copy = args.clone();
    dbg!(copy);

    let fp = args[1].clone();
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let input_lines = contents.split('\n');
}
