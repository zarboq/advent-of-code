use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(mut lines) = read_lines("./src/resources/d3e1") {
        let mut total_score: u16 = 0;
        while let Some(line) = lines.next() {
            let second_line = lines.next().unwrap();
            let third_line = lines.next().unwrap();
            let three_lines: [String; 3] = [line.unwrap(), second_line.unwrap(), third_line.unwrap()];
            // get badge from 3 lines/rucksacks
            let badge: u16 = get_badge(three_lines);
            // ASCII calculus
            if badge >= 97 {
                total_score += badge - 96;
            } else {
                println!("{}", badge);
                total_score += badge - 64 + 26;
            }
        }
        println!("Total points: {}", total_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_badge(rucksacks: [String; 3]) -> u16 {
    let mut object_counter: [u16; 123] = [0; 123];
    for i in 0..3 {
        let rucksack_length = rucksacks[i].chars().count();
        // add +1 for object in object counter
        // if object is present twice in someone ruckstack, add it only once
        let mut object_already_found: HashMap<u16, bool> = HashMap::new();
        for j in 0..(rucksack_length) {
            let current_object: u16 = rucksacks[i].chars().nth(j).unwrap() as u16;
            if !(*object_already_found.get(&current_object).unwrap_or(&false)) {
                object_already_found.insert(current_object, true);
                object_counter[current_object as usize] += 1;
            }
        }  
    }
    // if object is present 3 times return it
    for i in 0..(123) {
        if object_counter[i] == 3 {
            return i as u16
        }
    }
    return 0;
}