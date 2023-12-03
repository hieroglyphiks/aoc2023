use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::env;
use std::{char, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let copy = args.clone();
    dbg!(copy);

    let number_strings: HashMap<Vec<char>, usize> = HashMap::from([
        (vec!['z', 'e', 'r', 'o'], 0),
        (vec!['o', 'n', 'e'], 1),
        (vec!['t', 'w', 'o'], 2),
        (vec!['t', 'h', 'r', 'e', 'e'], 3),
        (vec!['f', 'o', 'u', 'r'], 4),
        (vec!['f', 'i', 'v', 'e'], 5),
        (vec!['s', 'i', 'x'], 6),
        (vec!['s', 'e', 'v', 'e', 'n'], 7),
        (vec!['e', 'i', 'g', 'h', 't'], 8),
        (vec!['n', 'i', 'n', 'e'], 9),
    ]);

    let fp = args[1].clone();
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let input_lines = contents.split('\n');

    let mut sum = 0;
    let mut cnt = 0;
    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        let characters: Vec<char> = line.chars().collect();


        cnt += 1;
        let mut first_num = String::new();
        let mut last_num = String::new();
        let mut first_found = false;
        let mut last_found = false;

        for (idx, _) in characters.iter().enumerate() {
            let numeric = is_numeric(characters.clone(), idx, number_strings.clone());
            match numeric {
                Ok(n_val) => {
                    if !first_found {
                        first_found = true;
                        first_num = n_val.to_string();
                    } else {
                        last_found = true;
                        last_num = n_val.to_string();
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }

        let mut combined = first_num.clone();
        if !last_found {
            combined.push_str(&first_num.clone());
        } else {
            combined.push_str(&last_num.clone());
        }

        let combined_num: u32 = combined.parse().expect("expected a numeric value");
        sum += combined_num;
    }

    println!("cnt: {cnt}, sum: {sum}");
}

fn is_numeric(
    characters: Vec<char>,
    idx: usize,
    numbers: HashMap<Vec<char>, usize>,
) -> Result<usize> {
    let character = characters[idx];
    if character.is_numeric() {
        return Ok(character.to_string().parse().expect("expected to parse"));
    } else {
        for (n_chars, n_value) in numbers.iter() {
            let ns_len = n_chars.len();

            if idx + ns_len > characters.len() {
                continue;
            }

            let sub_characters: Vec<char> = characters[idx..idx + ns_len].to_vec();

            if sub_characters == n_chars.clone() {
                return Ok(n_value.clone());
            }
        }
    }

    return Err(anyhow!("no numeric val found"));
}
