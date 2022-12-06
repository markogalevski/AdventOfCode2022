use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);

    let mut accumulator: i32 = 0;
    let mut accumulator_intersect: i32 = 0;
    for line in file.lines() {
        let line_contents = line.unwrap();
        let (left, right) = line_contents.split_once(',').unwrap();
        let (lower, upper) = left.split_once('-').unwrap();
        let lower_val: i32 = lower.parse().unwrap();
        let upper_val: i32 = upper.parse().unwrap();
        let left_set: HashSet<i32> = HashSet::from_iter(lower_val..=upper_val);
        let (lower, upper) = right.split_once('-').unwrap();
        let lower_val: i32 = lower.parse().unwrap();
        let upper_val: i32 = upper.parse().unwrap();
        let right_set: HashSet<i32> = HashSet::from_iter(lower_val..=upper_val);
        if is_sub_or_super(&left_set, &right_set) {
            accumulator += 1;
        }
        if is_non_empty_intersection(&left_set, &right_set) {
            accumulator_intersect += 1;
        }
    }
    println!("The total number of subsets is {accumulator}");
    println!("The total number of intersections is {accumulator_intersect}");
}

fn is_sub_or_super(left_set: &HashSet<i32>, right_set: &HashSet<i32>) -> bool {
    left_set.is_subset(right_set) || left_set.is_superset(right_set)
}

fn is_non_empty_intersection(left_set: &HashSet<i32>, right_set: &HashSet<i32>) -> bool {
    left_set.intersection(right_set).count() != 0
}
