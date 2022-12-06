use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d6") {
        let mut message_start: u16 = 0;
        for line in lines {
            message_start = process_message_start(line.unwrap());
        }
        println!("Message start: {}", message_start);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_message_start(message: String) -> u16 {
    let mut four_chars_set: HashSet<char> = HashSet::new();
    for i in 0..message.len() {
        if i > 13 {
            let char_to_remove = &message.chars().nth(i - 14).unwrap();
            let mut should_remove = true;
            for j in 0..14 {
                let char_min = message.chars().nth(i - j).unwrap();
                if *char_to_remove == char_min {
                    should_remove = false;
                }
            }
            if should_remove {
                four_chars_set.remove(char_to_remove);
            }
        }
        four_chars_set.insert(*&message.chars().nth(i).unwrap());
        if four_chars_set.len() == 14 {
            return (i + 1) as u16;
        }
    }
    return 0;
}
