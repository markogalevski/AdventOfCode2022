use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let mut file: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    if file.len() % 3 != 0 {
        file.append(
            &mut [""]
                .repeat(3 - file.len() % 3)
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }
    let mut iterator = file.iter();
    let mut peekable = iterator.by_ref().peekable();
    let mut badge_values: Vec<u32> = vec![];
    while peekable.by_ref().peek().is_some() {
        let rucksacks: Vec<&str> = peekable.by_ref().take(3).map(|x| x.as_str()).collect();
        let mut badge: Vec<char> = rucksacks[0]
            .chars()
            .filter(|c| {
                (rucksacks[1].contains(*c) || rucksacks[1].is_empty())
                    && (rucksacks[2].contains(*c) || rucksacks[2].is_empty())
            })
            .collect();
        badge.dedup();
        assert_eq!(badge.len(), 1);
        badge_values.push(char_to_u32(&badge[0]));
    }
    println!(
        "The sum of badge prios is: {}",
        badge_values.iter().sum::<u32>()
    );
}

fn char_to_u32(c: &char) -> u32 {
    let val = u32::from(*c);
    let lowercase_a = u32::from('a');
    let uppercase_a = u32::from('A');
    if val >= lowercase_a {
        val - lowercase_a + 1
    } else {
        val - uppercase_a + 26 + 1
    }
}
