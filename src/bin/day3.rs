use std::collections::HashSet;
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

fn main() {
    let input = include_str!("../../inputs/3.txt");

    let mut total_priority_sum = 0;
    let mut total_badge_sum = 0;
    let mut badge_buffer: Vec<String> = Vec::new();
    for line in input.lines() {
        let len = line.len();
        let part1 = &line[..(len / 2)];
        let part2 = &line[(len / 2)..];

        let diff: Vec<char> = diff_strings(part1, part2);
        let priority_sum = diff
            .iter()
            .map(|&c| calculate_priority(c))
            .reduce(|a, b| a + b);

        total_priority_sum.add_assign(priority_sum.unwrap_or(0));

        badge_buffer.push(line.to_string());

        if badge_buffer.len() == 3 {
            let buffer = badge_buffer.split_off(0);
            let d1 = diff_strings(buffer[0].as_str(), buffer[1].as_str());
            let d2 = diff_strings(buffer[1].as_str(), buffer[2].as_str());
            let d3 = diff_strings(
                d1.into_iter().collect::<String>().as_str(),
                d2.into_iter().collect::<String>().as_str(),
            );

            let priority = calculate_priority(*d3.get(0).unwrap());

            total_badge_sum.add_assign(priority);
        }
    }
    println!("part1: {}", total_priority_sum);
    println!("part2: {}", total_badge_sum);
}

/*
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
 */
