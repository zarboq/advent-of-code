use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d2") {
        let mut total_score: u16 = 0;
        let map_points = game_points();
        for line in lines {
            total_score += map_points.get(line.as_ref().unwrap()).unwrap();
        }
        println!("Total points: {}", total_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn game_points() -> HashMap<String, u16> {
    let mut score_map = HashMap::new();
    score_map.insert("A X".to_owned(), 4);
    score_map.insert("A Y".to_owned(), 8);
    score_map.insert("A Z".to_owned(), 3);
    score_map.insert("B X".to_owned(), 1);
    score_map.insert("B Y".to_owned(), 5);
    score_map.insert("B Z".to_owned(), 9);
    score_map.insert("C X".to_owned(), 7);
    score_map.insert("C Y".to_owned(), 2);
    score_map.insert("C Z".to_owned(), 6);
    return score_map;
}