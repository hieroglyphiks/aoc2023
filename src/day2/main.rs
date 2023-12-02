use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::env;
use std::{char, fs};

fn main() {
    part2();
}

fn part2() {
    let args: Vec<String> = env::args().collect();
    let copy = args.clone();
    dbg!(copy);

    let fp = args[1].clone();
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let input_lines = contents.split('\n');
    let mut power_sum = 0;
    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        // samples
        let line_parts: Vec<&str> = line.split(':').collect();
        if line_parts.len() != 2 {
            println!("line: {line}");
            panic!("invalid line fromat {line}")
        }

        let mut maximum_colors: HashMap<&str, u32> = HashMap::new();
        let samples: Vec<&str> = line_parts[1].split(';').collect();
        for sample in samples {
            let colored_samples: Vec<&str> = sample.trim().split(",").collect();

            for colored_sample in colored_samples {
                let split_sample: Vec<&str> = colored_sample.trim().split(" ").collect();
                if split_sample.len() != 2 {
                    println!("colored_sample: {colored_sample}");
                    panic!("invalid sample");
                }

                let count: u32 = split_sample[0].parse().expect("expected a numeric value");
                match maximum_colors.get(split_sample[1]) {
                    Some(maximum) => {
                        if count > *maximum {
                            maximum_colors.insert(split_sample[1], count.clone());
                        }
                    }
                    None => {
                        maximum_colors.insert(split_sample[1], count.clone());
                    }
                }
            }
        }

        let mut power = 0;
        for (_, maximum) in maximum_colors.iter() {
            if power == 0 {
                power = maximum.clone();
            } else {
                power = power * maximum;
            }
        }

        println!("power: {power} :  line: {line}");
        power_sum += power;
    }

    println!("power sum: {power_sum}");
}

fn part1() {
    let args: Vec<String> = env::args().collect();
    let copy = args.clone();
    dbg!(copy);

    let fp = args[1].clone();
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let limits: HashMap<&str, i32> = HashMap::from([("blue", 14), ("red", 12), ("green", 13)]);
    let mut valid_games = 0;

    let input_lines = contents.split('\n');
    for line in input_lines {
        if line.is_empty() {
            continue;
        }

        // samples
        let line_parts: Vec<&str> = line.split(':').collect();
        if line_parts.len() != 2 {
            println!("line: {line}");
            panic!("invalid line fromat {line}")
        }

        let mut valid_game = true;

        let samples: Vec<&str> = line_parts[1].split(';').collect();
        for sample in samples {
            let colored_samples: Vec<&str> = sample.trim().split(",").collect();
            let mut valid_sample = true;

            for colored_sample in colored_samples {
                let split_sample: Vec<&str> = colored_sample.trim().split(" ").collect();
                if split_sample.len() != 2 {
                    println!("colored_sample: {colored_sample}");
                    panic!("invalid sample");
                }

                let count: i32 = split_sample[0].parse().expect("expected a numeric value");
                match limits.get(split_sample[1]) {
                    Some(lc) => {
                        if count > *lc {
                            //println!("invalid game: {sample}");
                            valid_sample = false;
                            break;
                        }
                    }
                    None => {
                        println!("invalid sample color: {colored_sample}");
                        continue;
                    }
                }
            }

            if !valid_sample {
                valid_game = false;
                break;
            }
        }

        if valid_game {
            let game_id: Vec<&str> = line_parts[0].trim().split(' ').collect();
            if game_id.len() != 2 {
                println!("game_id: {line}");
                panic!("invalid game id");
            }

            let game: u32 = game_id[1]
                .parse()
                .expect("expected a numeric game id value");

            println!("valid game: {game} {valid_games}: {line}");

            valid_games += game;
        }
    }

    println!("valid games: {valid_games}");
}
