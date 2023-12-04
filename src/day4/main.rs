use core::panic;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("invalid args: expected: [part] [input filepath]")
    }

    let part = args[1].clone();
    let input_filepath = args[2].clone();

    match part.as_str() {
        "1" => {
            part1(input_filepath);
        }
        "2" => {
            part2(input_filepath);
        }
        _ => {
            panic!("invalid part: {part}")
        }
    }
}

fn part1(fp: String) {
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let mut input_lines: Vec<&str> = contents.split('\n').collect();
    input_lines.pop();

    let mut cards_sum = 0;

    for line in input_lines {
        // split the line
        let by_vline: Vec<&str> = line.trim().split("|").collect();
        //dbg!(by_vline.clone());

        let (card_name, winning_numbers_str) = by_vline[0].split_at(7);
        //println!("split card name: {card_name}, winning_numbers {winning_numbers_str}");
        let split_winning_nums: Vec<&str> = winning_numbers_str.trim().split(" ").collect();
        //dbg!(split_winning_nums.clone());

        let mut winning_numbers: HashSet<String> = HashSet::new();
        for w_num in split_winning_nums {
            winning_numbers.insert(w_num.to_string());
        }
        //dbg!(winning_numbers);

        let split_nums: Vec<&str> = by_vline[1].trim().split_whitespace().collect();
        //dbg!(split_nums.clone());

        let mut card_val = 0;
        for num_str in split_nums {
            if winning_numbers.contains(num_str) {
                //let num: usize = num_str.parse().expect("expected num_str to be a usize");

                if card_val == 0 {
                    card_val += 1;
                } else {
                    card_val = card_val * 2;
                }
            }
        }
        //dbg!(card_val);

        cards_sum += card_val;
    }

    println!("cards sum: {cards_sum}");
}

fn part2(fp: String) {
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let input_lines: Vec<&str> = contents.trim().split('\n').collect();

    let mut card_wins: HashMap<usize, usize> = HashMap::new();
    for line in input_lines.clone() {
        // split the line
        let by_vline: Vec<&str> = line.trim().split("|").collect();
        //dbg!(by_vline.clone());

        let parts: Vec<&str> = by_vline[0].split(":").collect();
        let winning_numbers_str = parts[1].trim().to_string();
        let card_name = parts[0].trim().to_string();

        //println!("split card name: {card_name}, winning_numbers {winning_numbers_str}");
        let split_winning_nums: Vec<&str> = winning_numbers_str.trim().split(" ").collect();
        //dbg!(split_winning_nums.clone());

        let mut winning_numbers: HashSet<String> = HashSet::new();
        for w_num in split_winning_nums {
            winning_numbers.insert(w_num.to_string());
        }
        //dbg!(winning_numbers);

        let split_nums: Vec<&str> = by_vline[1].trim().split_whitespace().collect();
        //dbg!(split_nums.clone());

        // build up the winning map
        for num_str in split_nums {
            if num_str == "" {
                continue;
            }

            if winning_numbers.contains(num_str) {
                let cname_ws_split: Vec<&str> = card_name.split_whitespace().collect();
                let cnumber: usize = cname_ws_split[1].parse().expect("expected card number");

                match card_wins.get(&cnumber) {
                    None => {
                        card_wins.insert(cnumber.clone(), 1);
                    }
                    Some(wins) => {
                        card_wins.insert(cnumber.clone(), *wins + 1);
                    }
                }
            }
        }
    }

    let ilines = input_lines.len();
    dbg!(input_lines);
    println!("input lines len: {ilines}");

    let mut retval = 0;
    for card in 1..ilines + 1 {
        let r = recurse(card_wins.clone(), card);
        retval += r;

        println!("ret: {retval}, r: {r}");
    }

    println!("retval: {retval}")
}

fn recurse(map: HashMap<usize, usize>, card: usize) -> usize {
    let mut copies_won = 1;
    match map.get(&card) {
        None => {
            return copies_won;
        }
        Some(wins) => {
            let end_card_idx = card + wins;
            for copied_card in card + 1..end_card_idx + 1 {
                copies_won += recurse(map.clone(), copied_card);
            }
        }
    }

    return copies_won;
}
