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
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_size = self.trees[x][y];
        if x == 0 || y == 0 || x == 98 || y == 98 {
            return true;
        }

        let mut covered_sides = [false; 4];
        // iterate on rows, constant y
        for i in 0..self.trees.len() {
            if i != x {
                let current_tree = self.trees[i][y];
                if current_tree >= tree_size {
                    if i < x {
                        covered_sides[0] = true;
                    }
                    if i > x {
                        covered_sides[1] = true;
                    }
                }
            }
        }
        // iterate on columns, constant x
        for j in 0..self.trees[0].len() {
            if j != y {
                let current_tree = self.trees[x][j];
                if current_tree >= tree_size {
                    if j < y {
                        covered_sides[2] = true;
                    }
                    if j > y {
                        covered_sides[3] = true;
                    }
                }
            }
        }
        for sides in covered_sides {
            if !sides {
                return !sides;
            }
        }
        return false;
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

        let mut visible_trees = 0;
        for x in 0..99 {
            for y in 0..99 {
                if forest.is_visible(x, y) {
                    visible_trees += 1;
                }
            }
        }
        println!("{}", visible_trees);
    }
}
