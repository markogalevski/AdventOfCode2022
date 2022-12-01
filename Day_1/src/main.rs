use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = env!("CARGO_MANIFEST_DIR");
    let file = File::open(args[1].clone()).unwrap();
    let file = BufReader::new(file);

    struct Elf(i32, i32);
    let mut elves = Vec::<Elf>::new();
    let mut elf_index = 1;
    let mut accumulator = 0;
    for line in file.lines() {
        let line_contents = line.unwrap();
        if !line_contents.is_empty() {
            let line_parsed: i32 = line_contents.parse().unwrap();
            accumulator += line_parsed;
        } else {
            elves.push(Elf(accumulator, elf_index));
            accumulator = 0;
            elf_index += 1;
        }
    }
    elves.sort_by(|a, b| b.0.cmp(&a.0));

    accumulator = 0;
    for i in 0..3 {
        println!("The next most calories an elf has is {}", elves[i].0);
        println!("That lucky elf is elf number {}", elves[i].1);
        accumulator += elves[i].0;
    }
    println!("The total calories they are carrying is {accumulator}!");
}
