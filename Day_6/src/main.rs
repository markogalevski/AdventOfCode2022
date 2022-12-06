use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let mut file = BufReader::new(file);
    let mut stream = String::new();
    file.read_line(&mut stream).ok();
    println!("{}", find_start_of_message(&stream));
    println!("{}", find_start_of_packet(&stream));
}

fn find_start_of_message(stream: &str) -> usize {
    find_x_unique(stream, 14)
}

fn find_start_of_packet(stream: &str) -> usize {
    find_x_unique(stream, 4)
}

fn find_x_unique(stream: &str, x: usize) -> usize {
    for i in 0..stream.len() {
        let slice = &stream[i..i + x];
        if slice.chars().combinations(2).all(|pair| pair[0] != pair[1]) {
            return i + x;
        }
    }
    return 0;
}
