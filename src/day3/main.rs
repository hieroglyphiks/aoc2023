use core::panic;
use std::cmp::{max, min};
use std::collections::HashMap;
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

    let mut input_grid = Grid::from_lines(input_lines);
    input_grid.print();

    let parts_sum = input_grid.parts_sum();
    println!("parts sum: {parts_sum}");
}

fn part2(fp: String) {
    let contents = fs::read_to_string(fp).expect("Should have been able to read the input file");

    let mut input_lines: Vec<&str> = contents.split('\n').collect();
    input_lines.pop();

    let mut input_grid = Grid::from_lines(input_lines);
    input_grid.print();

    let parts_sum = input_grid.gear_ratio();
    println!("parts sum: {parts_sum}");
}

struct Grid {
    values: Vec<Vec<i32>>,
}

impl Grid {
    fn from_lines(lines: Vec<&str>) -> Self {
        let mut result = Self { values: Vec::new() };

        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            let mut values_line: Vec<i32> = Vec::new();

            for (_, c) in chars.iter().enumerate() {
                if c.is_numeric() {
                    let num: i32 = c.to_string().parse().expect("expected a numeric value");
                    values_line.push(num);
                } else if *c == '.' {
                    values_line.push(-1);
                } else if *c == '*' {
                    values_line.push(-2);
                } else {
                    values_line.push(-3);
                }
            }
            result.values.push(values_line);
        }

        return result;
    }

    fn print(&mut self) {
        let mut p = String::new();
        for line in self.values.clone() {
            let mut s = String::new();
            for i in line {
                s.push_str(i.to_string().as_str());
                s.push_str("|");
            }
            s.push_str("\n");
            p.push_str(s.as_str());
        }
        println!("{p}");
    }

    fn parts_sum(&mut self) -> usize {
        let mut ps = 0;
        for (idy, line) in self.values.clone().iter().enumerate() {
            println!("{idy}");
            for (idx, i) in line.iter().enumerate() {
                if idx > 0 {
                    if line[idx - 1] >= 0 {
                        continue;
                    }
                }

                let mut num_str = String::new();
                if *i > 0 {
                    let mut idx2 = idx.clone();
                    while idx2 < line.len() {
                        if line[idx2] < 0 {
                            break;
                        }

                        num_str.push_str(line[idx2].to_string().as_str());
                        idx2 += 1;
                    }

                    println!("num_str: {num_str}");
                    if self.is_part(idy, idx, max(0, idx2 - 1)) {
                        let num: usize = num_str.parse().expect("expected a number");
                        println!("part: {num}");
                        ps += num;
                    }
                }
            }
        }

        return ps;
    }

    fn is_part(&mut self, idy: usize, idx1: usize, idx2: usize) -> bool {
        println!("idy: {idy}, idx1: {idx1}, idx2: {idx2}");

        let mut xmin = 0;
        if idx1 > 0 {
            xmin = idx1 - 1;
        }

        // top line
        if idy > 0 {
            let line = self.values[idy - 1].clone();
            let startx = xmin;
            let endx = min(line.len() - 1, idx2 + 1);

            for x in startx..endx + 1 {
                if line[x] <= -2 {
                    return true;
                }
            }
        }

        // current line
        {
            let line = self.values[idy].clone();
            let startx = xmin;
            let endx = min(line.len() - 1, idx2 + 1);

            if line[startx] <= -2 {
                return true;
            }

            if line[endx] <= -2 {
                return true;
            }
        }

        // bottom line
        if idy < self.values.len() - 1 {
            let line = self.values[idy + 1].clone();
            let startx = xmin;
            let endx = min(line.len() - 1, idx2 + 1);

            println!("start: {startx}, end:{endx}");

            for x in startx..endx + 1 {
                if line[x] <= -2 {
                    return true;
                }
            }
        }

        return false;
    }

    fn gear_ratio(&mut self) -> usize {
        let mut gear_locs: HashMap<Coordinate, Vec<GridNumber>> = HashMap::new();
        for (idy, line) in self.values.clone().iter().enumerate() {
            println!("idy: {idy}");
            for (idx, i) in line.iter().enumerate() {
                if idx > 0 {
                    if line[idx - 1] >= 0 {
                        continue;
                    }
                }

                let mut num_str = String::new();
                if *i > 0 {
                    let mut idx2 = idx.clone();
                    while idx2 < line.len() {
                        if line[idx2] < 0 {
                            break;
                        }

                        num_str.push_str(line[idx2].to_string().as_str());
                        idx2 += 1;
                    }

                    let num: usize = num_str.parse().expect("expected num string to be a num");
                    let gn = GridNumber::new(idy, idx, idx2 - 1, num);
                    match self.touches_gear(gn.clone()) {
                        Some(c) => {
                            let y = c.y;
                            let x = c.x;
                            println!("gear -> x: {x}, y: {y}");

                            if !gear_locs.contains_key(&c) {
                                gear_locs.insert(c.clone(), vec![gn.clone()]);
                            } else {
                                let mut gns =
                                    gear_locs.get(&c).expect("expected a gear vector").clone();
                                gns.push(gn.clone());
                                gear_locs.insert(c.clone(), gns);
                            }
                        }
                        None => continue,
                    }
                }
            }
        }

        let mut gs = 0;
        for (coord, gns) in gear_locs.iter() {
            if gns.len() == 2 {
                dbg!(coord);
                let mut gr = 1;
                for gn in gns {
                    gr = gr * gn.value;
                }
                println!("gr: {gr}");
                gs += gr;
            }
        }

        return gs;
    }

    fn touches_gear(&mut self, gn: GridNumber) -> Option<Coordinate> {
        /*let y = gn.y;
        let x1 = gn.xstart;
        let x2 = gn.xend;
        println!("gn: y {y}, x1 {x1}, x2 {x2}");
        */

        let mut xmin = 0;
        if gn.xstart > 0 {
            xmin = gn.xstart - 1;
        }

        // top line
        if gn.y > 0 {
            let line = self.values[gn.y - 1].clone();
            let startx = xmin;
            let endx = min(line.len() - 1, gn.xend + 1);

            for idx in startx..endx + 1 {
                if line[idx] == -2 {
                    return Some(Coordinate {
                        y: gn.y - 1,
                        x: idx,
                    });
                }
            }
        }

        // current line
        {
            let line = self.values[gn.y].clone();
            let startx = xmin;
            let endx = min(line.len() - 1, gn.xend + 1);

            if line[startx] == -2 {
                return Some(Coordinate { y: gn.y, x: startx });
            }

            if line[endx] == -2 {
                return Some(Coordinate { y: gn.y, x: endx });
            }
        }

        // bottom line
        if gn.y < self.values.len() - 1 {
            let line = self.values[gn.y + 1].clone();
            let startx = xmin;
            let endx = min(line.len() - 1, gn.xend + 1);

            for idx in startx..endx + 1 {
                if line[idx] == -2 {
                    return Some(Coordinate {
                        y: gn.y + 1,
                        x: idx,
                    });
                }
            }
        }

        return None;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct GridNumber {
    y: usize,
    xstart: usize,
    xend: usize,
    value: usize,
}

impl GridNumber {
    fn new(y1: usize, x1: usize, x2: usize, val: usize) -> Self {
        return Self {
            y: y1,
            xstart: x1,
            xend: x2,
            value: val,
        };
    }

    fn contains(&mut self, y: usize, x: usize) -> bool {
        if y != self.y {
            return false;
        }

        if x < self.xstart || x > self.xend {
            return false;
        }

        return true;
    }
}
