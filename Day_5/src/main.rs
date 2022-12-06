use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn is_bulk(args: &[String]) -> bool {
    if args.len() >= 3 {
        args[2] == "bulk"
    } else {
        false
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let file: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut iter = file.split(|x| x == "");
    let mut peekable = iter.clone().peekable();
    let num_slots = peekable.peek().unwrap()[0].len() / 4 + 1;
    let mut slots: Vec<VecDeque<char>> = vec![VecDeque::new(); num_slots];
    println!("The number of deques allocated is {:?}", slots.len());

    /* build initial state */
    for line in iter.next().unwrap().split(|x| x == "") {
        for sub_line in line {
            let sub_line_chars: Vec<char> = sub_line.chars().collect();
            for (i, thing) in sub_line.match_indices('[') {
                slots[i / 4].push_front(sub_line_chars[i + 1]);
            }
        }
    }
    println!("Starting state is: \n{:?}", slots);
    if is_bulk(&args) {
        println!("Moving boxes in bulk");
    } else {
        println!("Moving boxes individually");
    }
    for line in iter.next().unwrap() {
        let a: Vec<&str> = line.split(" ").collect();
        let num_moves: usize = a[1].parse().unwrap();
        if is_bulk(&args) {
            move_packages_bulk(
                &mut slots,
                num_moves,
                a[3].parse::<usize>().unwrap() - 1,
                a[5].parse::<usize>().unwrap() - 1,
            );
        } else {
            move_packages_singly(
                &mut slots,
                num_moves,
                a[3].parse::<usize>().unwrap() - 1,
                a[5].parse::<usize>().unwrap() - 1,
            );
        }
    }
    let mut final_compact = String::new();
    for deque in slots.iter_mut() {
        final_compact.push(deque.pop_back().unwrap());
    }
    println!("{final_compact}");
}

fn move_packages_singly(slots: &mut Vec<VecDeque<char>>, how_many: usize, from: usize, to: usize) {
    for _ in 0..how_many {
        match slots[from].pop_back() {
            Some(package) => slots[to].push_back(package),
            None => (),
        }
    }
}

fn move_packages_bulk(slots: &mut Vec<VecDeque<char>>, how_many: usize, from: usize, to: usize) {
    let num_packages = slots[from].len();
    let pile = slots[from]
        .drain(num_packages - how_many..)
        .collect::<Vec<char>>();
    for package in pile.iter() {
        slots[to].push_back(*package);
    }
}
