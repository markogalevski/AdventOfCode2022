#[allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Entries {
    DirEntry(String),
    FileEntry(i32, String),
}

impl FromStr for Entries {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let entry_split: Vec<&str> = s.split(" ").collect();
        match entry_split[0] {
            "dir" => Ok(Entries::DirEntry(entry_split[1].to_string())),
            &_ => Ok(Entries::FileEntry(
                entry_split[0].clone().parse::<i32>().unwrap(),
                entry_split[1].to_string(),
            )),
        }
    }
}

#[derive(Clone, Debug)]
struct Folder {
    name: String,
    parent: Option<Box<Folder>>,
    subfolders: Vec<Box<Folder>>,
    file_sizes: Vec<i32>,
}

impl Folder {
    fn sum_size(&self) -> i32 {
        let local_sum: i32 = self.file_sizes.iter().sum();

        local_sum
            + self
                .subfolders
                .iter()
                .map(|subfolder| subfolder.sum_size())
                .sum::<i32>()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = BufReader::new(File::open(args[1].clone()).unwrap());
    let file_contents = file.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut file_contents = file_contents.iter().peekable();
    let mut folder_structure = vec![Box::new(Folder {
        name: "/".to_string(),
        parent: None,
        subfolders: Vec::<Box<Folder>>::new(),
        file_sizes: Vec::<i32>::new(),
    })];
    let mut current_folder: Option<&mut Box<Folder>> = None;
    while let Some(line) = file_contents.next() {
        println!("{}", line);
        let command: Vec<&str> = line.split(' ').collect();
        match command[0] {
            "$" => {
                match command[1] {
                    "ls" => {
                        /*It's a list command.
                        The next lines until a $ will contain data to handle  */
                        while file_contents.peek().is_some()
                            && !file_contents.peek().unwrap().starts_with("$")
                        {
                            match Entries::from_str(file_contents.next().unwrap()) {
                                Ok(entry) => match entry {
                                    Entries::FileEntry(size, _) => {
                                        /* Add file size to current file_sizes array */
                                        println!(
                                            "Definitely adding file sizes from {:?}",
                                            entry.clone()
                                        );
                                        current_folder.unwrap().file_sizes.push(size);
                                    }
                                    Entries::DirEntry(name) => {
                                        /* Create and add dir (if not already there) to subfolders */
                                        if !current_folder
                                            .unwrap()
                                            .subfolders
                                            .iter()
                                            .any(|subfolder| subfolder.name == name)
                                        {
                                            println!("Definitely adding dir from {:?}", name);
                                        } else {
                                            println!("already found it! {:?}", name);
                                        }
                                    }
                                },
                                Err(_) => (),
                            }
                        }
                    }
                    "cd" => {
                        /*
                         *It's a change directory command.
                         *command[2] will navigate around our structure
                         */
                        for folder in folder_structure.iter_mut() {
                            if command[2] == ".." {
                                if folder.parent.is_some() {
                                    current_folder = Some(folder);
                                } else {
                                    current_folder = None;
                                }
                                break;
                            } else if folder.name == command[2] {
                                current_folder = Some(folder);
                                break;
                            }
                        }
                    }
                    &_ => (), // ?????
                }
            }
            &_ => (),
        }
    }
}
