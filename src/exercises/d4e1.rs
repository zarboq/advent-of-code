use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d4e1") {
        let mut total_score: u16 = 0;
        for line in lines {
            if full_overlap(line.unwrap()) {
                total_score += 1;
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

fn full_overlap(assignements: String) -> bool {
    let mut splitted_assignements = assignements.split(",");
    let assignement1_first_section: u8;
    let assignement1_last_section: u8;
    let assignement2_first_section: u8;
    let assignement2_last_section: u8;

    let assignement = splitted_assignements.next().unwrap();
    let mut splitted_sections = assignement.split("-");
    assignement1_first_section = splitted_sections.next().unwrap().parse().unwrap();
    assignement1_last_section = splitted_sections.next().unwrap().parse().unwrap();

    let assignement2 = splitted_assignements.next().unwrap();
    let mut splitted_sections2 = assignement2.split("-");
    assignement2_first_section = splitted_sections2.next().unwrap().parse().unwrap();
    assignement2_last_section = splitted_sections2.next().unwrap().parse().unwrap();

    if (assignement1_first_section >= assignement2_first_section && assignement1_last_section <= assignement2_last_section)
        || (assignement2_first_section >= assignement1_first_section && assignement2_last_section <= assignement1_last_section) {
        return true;
    }
    false
}