use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d1") {
        let mut top_num_of_cals: i32 = 0;
        let mut cur_elf_num_of_cals: i32 = 0;
        for line in lines {
            if line.as_ref().unwrap().len() == 0 {
                if cur_elf_num_of_cals > top_num_of_cals {
                    top_num_of_cals = cur_elf_num_of_cals;
                }
                cur_elf_num_of_cals = 0;
            } else {
                cur_elf_num_of_cals += line.unwrap().trim().parse::<i32>().unwrap();
            }
        }
        println!("highest cal is: {}", top_num_of_cals);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}