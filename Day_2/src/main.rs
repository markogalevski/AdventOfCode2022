use std::convert::From;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RPS {
    fn from(string: &str) -> Self {
        match string {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            &_ => RPS::Rock,
        }
    }
}

trait Beats {
    fn beats(&self) -> Self;
    fn loses_to(&self) -> Self;
}

impl Beats for RPS {
    fn beats(&self) -> Self {
        match *self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match *self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

trait ToPoints {
    fn to_points(&self) -> i32;
}

impl ToPoints for RPS {
    fn to_points(&self) -> i32 {
        match *self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = env!("CARGO_MANIFEST_DIR");
    let file = File::open(args[1].clone()).unwrap();
    let file = BufReader::new(file);
    let strategy = args[2].clone();

    let mut accumulator: i32 = 0;
    for line in file.lines() {
        let line_contents = line.unwrap();
        let moves: Vec<&str> = line_contents.as_str().split(' ').collect();
        //let (theirs, mine) = play_xyz_to_abc(moves);
        let (theirs, mine) = if strategy.to_lowercase() == "wld" {
            play_xyz_to_wld(moves)
        } else {
            play_xyz_to_abc(moves)
        };
        if mine.beats() == theirs {
            accumulator += 6;
        } else if theirs.beats() == mine {
            accumulator += 0;
        } else {
            accumulator += 3;
        }
        accumulator += mine.to_points();
    }
    println!("Your final score is: {accumulator}");
}

fn play_xyz_to_abc(moves: Vec<&str>) -> (RPS, RPS) {
    (RPS::from(moves[0]), RPS::from(moves[1]))
}

fn win_lose_draw(theirs: RPS, strategy: &str) -> RPS {
    match strategy {
        "X" => theirs.beats(),
        "Y" => theirs,
        "Z" => theirs.loses_to(),
        &_ => theirs,
    }
}

fn play_xyz_to_wld(moves: Vec<&str>) -> (RPS, RPS) {
    (
        RPS::from(moves[0]),
        win_lose_draw(RPS::from(moves[0]), moves[1]),
    )
}
