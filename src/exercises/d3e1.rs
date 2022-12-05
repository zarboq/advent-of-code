use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d3") {
        let mut total_score: u16 = 0;
        for line in lines {
            let present_twice_object: u16 = object_points(line.unwrap()) as u16;
            // calculate object value with ASCII
            if present_twice_object >= 97 {
                total_score += present_twice_object - 96;
            } else {
                total_score += present_twice_object - 64 + 26;
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

fn object_points(rucksack: String) -> char {
    let mut present_objects: HashMap<char, bool> = HashMap::new();
    let rucksack_length = rucksack.chars().count();
    // store all the objects present in the second half
    for i in 0..(rucksack_length/2) {
        present_objects.insert(rucksack.chars().nth(rucksack_length - i - 1).unwrap(), true);
    }
    // if object also present in the first one return it
    for i in 0..(rucksack_length/2) {
        let current_object: char = rucksack.chars().nth(i).unwrap();
        if *present_objects.get(&current_object).unwrap_or(&false) {
            return current_object;
        }
    }
    return '0';
}