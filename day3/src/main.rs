use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::AddAssign;

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
    let mut total_badge_sum = 0;
    let mut badge_buffer: Vec<String> = Vec::new();
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

        total_priority_sum.add_assign(priority_sum.unwrap_or(0));

        println!(
            "part1 = {}, part2 = {}, diff = {:?}, priority_sum = {:?}",
            part1, part2, diff, priority_sum
        );

        badge_buffer.push(line);

        if badge_buffer.len() == 3 {
            let buffer = badge_buffer.split_off(0);
            let d1 = diff_strings(buffer[0].as_str(), buffer[1].as_str());
            let d2 = diff_strings(buffer[1].as_str(), buffer[2].as_str());
            let d3 = diff_strings(
                d1.into_iter().collect::<String>().as_str(),
                d2.into_iter().collect::<String>().as_str(),
            );

            let priority = calculate_priority(*d3.get(0).unwrap());

            println!("badge: {:?}", d3);
            total_badge_sum.add_assign(priority);
        }
    }
    println!("total_priority_sum: {}", total_priority_sum);
    println!("total_badge_sum: {}", total_badge_sum);

    Ok(())
}
