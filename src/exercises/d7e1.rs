use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type DirectoryNode = Option<Box<Directory>>;

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    parent: DirectoryNode,
    childs: HashMap<String, Directory>,
    files: HashMap<String, FileEx>,
}

#[derive(Clone, Debug)]
struct FileEx {
    name: String,
    size: u32,
}

impl Directory {
    fn new(name: String, parent: DirectoryNode) -> Self {
        Directory {
            name: name,
            parent: parent,
            childs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn compute_size(&self) -> u32 {
        let mut total_size = 0;
        for file in self.files.values() {
            total_size += file.size;
        }
        return total_size;
    }
}

impl FileEx {
    fn new(name: String, size: u32) -> Self {
        FileEx {
            name: name,
            size: size,
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d7") {
        let directories: HashMap<String, Directory> = parse_file(lines);
        let mut total_size = 0;
        for directory in directories.values() {
            let size = directory.clone().compute_size();
            if size <= 100000 {
                total_size += size;
            }
        }
        println!("Total points: {}", total_size);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_file(lines: io::Lines<io::BufReader<File>>) -> HashMap<String, Directory> {
    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut current_dir: &Directory = &Directory::new("/".to_owned(), None);
    for line in lines {
        let mut parts = line.as_ref().unwrap().split_whitespace();
        let first_element = parts.next().unwrap();
        if first_element == "$" {
            let command = parts.next().unwrap();
            if command == "cd" {
                let directory_name = parts.next().unwrap().to_owned();
                // Replace the reference to current_dir with a mutable reference
                println!("{}", directory_name);
                current_dir = directories.get_mut(&directory_name).unwrap();
            } else if command == "ls" {
                continue;
            }
        } else if first_element == "dir" {
            // if directory
            let directory_name = parts.next().unwrap().to_owned();
            let new_directory = Directory::new(
                (&*directory_name).to_owned(),
                Some(Box::new(current_dir.clone())),
            );
            // add it to the list if not already present
            directories
                .entry((&*directory_name).to_owned())
                .or_insert_with(|| new_directory.clone());
            // add it to current_dir childs
            current_dir
                .childs
                .entry((&*directory_name).to_owned())
                .or_insert_with(|| new_directory.clone());
        } else {
            // get file size from string
            println!("{}", first_element);
            let file_size = if let Ok(file_size) = first_element.parse::<u32>() {
                file_size
            } else {
                panic!()
            };
            // create file
            let file = FileEx::new(first_element.to_owned(), file_size);
            // add it to current_dir files
            current_dir
                .files
                .entry(first_element.to_owned())
                .or_insert_with(|| file);
        }
    }
    return directories;
}
