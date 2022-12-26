use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn diff_strings(a: &str, b: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    let set: HashSet<char> = a.chars().collect();
    b.chars().for_each(|c| {
        if set.contains(&c) && !result.contains(&c) {
            result.push(c)
        }
    });

    result
}

fn calculate_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return (c as u32) - 96;
    }
    if c.is_ascii_uppercase() {
        return (c as u32) - 38;
    }
    0
}

fn main() -> io::Result<()> {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);
    let mut total_priority_sum = 0;
    for line in reader.lines() {
        let line = line?;
        let len = line.len();
        let part1 = &line[..(len / 2)];
        let part2 = &line[(len / 2)..];

        let diff: Vec<char> = diff_strings(part1, part2);
        let priority_sum = diff
            .iter()
            .map(|&c| calculate_priority(c))
            .reduce(|a, b| a + b);

        total_priority_sum = total_priority_sum + priority_sum.unwrap_or(0);

        println!(
            "part1 = {}, part2 = {}, diff = {:?}, priority_sum = {:?}",
            part1, part2, diff, priority_sum
        )
    }
    println!("total_priority_sum: {}", total_priority_sum);

    Ok(())
}
