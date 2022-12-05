use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Stack {
    crates: VecDeque<char>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            crates: VecDeque::new(),
        }
    }

    fn add_crates(&mut self, crates_to_add: VecDeque<char>) {
        let crates_to_add_len = crates_to_add.len();
        for i in 0..crates_to_add_len {
            self.crates.push_back(crates_to_add[crates_to_add_len - i - 1]);
        }
    }

    fn remove_n_crates(&mut self, n: u8) -> VecDeque<char> {
        let removed_crates = self.crates.split_off(self.crates.len() - n as usize);
        return removed_crates;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn stacks_from_text(lines: io::Lines<io::BufReader<File>>) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = Vec::new();
    let mut first = false;
    for line in lines {
        if !first {
            stacks = vec![Stack::new(); (line.as_ref().unwrap().len() + 1) / 4];
            first = true;
        }
        if line.as_ref().unwrap().chars().nth(1).unwrap() == '1' {
            break;
        }
        let mut x = 0;
        for i in (1..line.as_ref().unwrap().len()).step_by(4) {
            let cur_crate: char = line.as_ref().unwrap().chars().nth(i).unwrap();
            if cur_crate != ' ' {
                stacks[x].crates.push_front(cur_crate);
            }
            x += 1;
        }
    }
    return stacks
}

fn rearrange_stacks(lines: io::Lines<io::BufReader<File>>, mut stacks: Vec<Stack>) -> Vec<Stack> {
    let mut begin_reading = false;
    for line in lines {
        if line.as_ref().unwrap().len() == 0 { continue }
        if !begin_reading {
            if line.as_ref().unwrap().chars().nth(1).unwrap() == '1' {
                begin_reading = true;
            }
            continue;
        }

        let mut parts = line.as_ref().unwrap().split_whitespace();
        parts.next();
        let a = if let Ok(a) = parts.next().unwrap().parse::<u8>() { a } else { panic!() };
        parts.next();
        let b = if let Ok(b) = parts.next().unwrap().parse::<u8>() { b } else { panic!() };
        parts.next();
        let c = if let Ok(c) = parts.next().unwrap().parse::<u8>() { c } else { panic!() };

        let removed_crates = stacks[(b - 1) as usize].remove_n_crates(a);
        stacks[(c - 1) as usize].add_crates(removed_crates);
    }
    return stacks;
}

fn main() {
    let mut stacks = Vec::new();
    if let Ok(lines) = read_lines("./src/resources/d5") {
        let mut total_score: u16 = 0;
        stacks = stacks_from_text(lines);
    }
    if let Ok(lines) = read_lines("./src/resources/d5") {
        let mut total_score: u16 = 0;
        stacks = rearrange_stacks(lines, stacks);
    }
    for stack in stacks {
        println!("{}", stack.crates[stack.crates.len() - 1]);
    }
}