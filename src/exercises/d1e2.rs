use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d1") {
        let mut top3_arr: [i32; 250] = [0; 250];
        let mut cur_elf_num_of_cals: i32 = 0;
        let mut i = 0;
        for line in lines {
            if line.as_ref().unwrap().len() == 0 {
                top3_arr[i] = cur_elf_num_of_cals;
                i += 1;
                cur_elf_num_of_cals = 0;

            } else {
                cur_elf_num_of_cals += line.unwrap().trim().parse::<i32>().unwrap();
            }
        }
        // lazy :)
        top3_arr.sort_by(|a, b| b.cmp(a));
        let sum = top3_arr[0] + top3_arr[1] + top3_arr[2];
        println!("highest cal is: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}