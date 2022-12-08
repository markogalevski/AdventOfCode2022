use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = BufReader::new(File::open(args[1].clone()).unwrap());
    let file_contents: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    for line in file_contents.iter() {
        let row: Vec<u32> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
        matrix.push(row);
    }
    println!(
        "There's a total of {} visible trees!",
        count_visible_trees(&matrix)
    );

    println!(
        "The highest view score tree has a view score of {}",
        calc_max_view_score(&matrix)
    );
}

fn count_visible_trees(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut num_visible_trees = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if row[..j].iter().all(|element| element < value) {
                num_visible_trees += 1;
                continue;
            }
            if row[(j + 1)..].iter().all(|element| element < value) {
                num_visible_trees += 1;
                continue;
            }
            if matrix[..i]
                .iter()
                .map(|row| row[j])
                .all(|element| element < *value)
            {
                num_visible_trees += 1;
                continue;
            }

            if matrix[(i + 1)..]
                .iter()
                .map(|row| row[j])
                .all(|element| element < *value)
            {
                num_visible_trees += 1;
                continue;
            }
        }
    }
    num_visible_trees
}

fn calc_max_view_score(matrix: &Vec<Vec<u32>>) -> usize {
    let mut max_view_score = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let mut local_view_score = 1;
            let mut reversed_row = row.clone();
            reversed_row.reverse();
            let mut all_the_way = true;
            for (i, element) in reversed_row[(row.len() - j)..].iter().enumerate() {
                if element >= value {
                    local_view_score *= i + 1;
                    all_the_way = false;
                    break;
                }
            }
            if all_the_way {
                local_view_score *= reversed_row[(row.len() - j)..].len();
            }

            all_the_way = true;
            for (i, element) in row[(j + 1)..].iter().enumerate() {
                if element >= value {
                    local_view_score *= i + 1;
                    all_the_way = false;
                    break;
                }
            }
            if all_the_way {
                local_view_score *= row[(j + 1)..].len();
            }

            all_the_way = true;
            let mut reversed_column_slice: Vec<u32> =
                matrix[..i].iter().map(|row| row[j]).collect();
            reversed_column_slice.reverse();
            for (x, element) in reversed_column_slice.iter().enumerate() {
                if element >= value {
                    local_view_score *= x + 1;
                    all_the_way = false;
                    break;
                }
            }
            if all_the_way {
                local_view_score *= reversed_column_slice.len();
            }
            all_the_way = true;
            let column_slice: Vec<u32> = matrix[i + 1..].iter().map(|row| row[j]).collect();
            for (x, element) in column_slice.iter().enumerate() {
                if element >= value {
                    local_view_score *= x + 1;
                    all_the_way = false;
                    break;
                }
            }
            if all_the_way {
                local_view_score *= column_slice.len();
            }
            max_view_score = std::cmp::max(max_view_score, local_view_score);
        }
    }
    max_view_score
}
