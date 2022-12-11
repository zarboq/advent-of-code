use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    fn new() -> Self {
        Forest {
            trees: vec![vec![0; 99]; 99],
        }
    }

    // check if 4 sides of a tree are covered by higher tree
    fn compute_scenic_score(&self, x: usize, y: usize) -> u32 {
        let tree_size = self.trees[x][y];
        let mut scenic_score = [0, 0, 0, 0];
        let mut i = x;
        while i >= 0 {
            if i != x {
                let current_tree = self.trees[i][y];
                if current_tree < tree_size {
                    scenic_score[0] += 1;
                } else if current_tree >= tree_size {
                    scenic_score[0] += 1;
                    break;
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        let mut i = x;
        while i <= 98 {
            if i != x {
                let current_tree = self.trees[i][y];
                if current_tree < tree_size {
                    scenic_score[1] += 1;
                } else if current_tree >= tree_size {
                    scenic_score[1] += 1;
                    break;
                }
            }
            i += 1;
        }

        let mut j = y;
        while j >= 0 {
            if j != y {
                let current_tree = self.trees[x][j];
                println!("{} {}", x, j);
                println!("{}", tree_size);
                println!("{}", current_tree);

                if current_tree < tree_size {
                    scenic_score[2] += 1;
                } else if current_tree >= tree_size {
                    scenic_score[2] += 1;
                    break;
                }
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }

        let mut j = y;
        while j <= 98 {
            if j != y {
                let current_tree = self.trees[x][j];
                if current_tree < tree_size {
                    scenic_score[3] += 1;
                } else if current_tree >= tree_size {
                    scenic_score[3] += 1;
                    break;
                }
            }
            j += 1;
        }

        return scenic_score[0] * scenic_score[1] * scenic_score[2] * scenic_score[3];
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d8") {
        let collected_lines: Vec<String> = lines
            .filter_map(|result| match result {
                Ok(line) => Some(line),
                Err(_) => None,
            })
            .collect();
        let mut forest: Forest = Forest::new();
        let mut i = 0;
        let mut j = 0;
        // build forest
        for line in collected_lines {
            for tree in line.chars() {
                forest.trees[i][j] = tree.to_digit(10).unwrap();
                j += 1;
            }
            i += 1;
            j = 0;
        }

        let mut max_scenic = 0;
        for x in 0..99 {
            for y in 0..99 {
                let scenic = forest.compute_scenic_score(x, y);
                if scenic > max_scenic {
                    max_scenic = scenic;
                }
            }
        }

        println!("{}", max_scenic);
    }
}
