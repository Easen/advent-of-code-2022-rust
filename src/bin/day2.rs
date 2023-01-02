use std::ops::AddAssign;

fn strategy_1(inputs: Vec<&str>) -> u32 {
    let score_shape = match inputs[1] {
        "X" => 1, // rock
        "Y" => 2, // paper
        "Z" => 3, // scissors
        _ => 0,
    };

    let score_match = match inputs[0] {
        // rock
        "A" => match inputs[1] {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => 0,
        },
        // paper
        "B" => match inputs[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        },
        // scissors
        "C" => match inputs[1] {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => 0,
        },
        _ => 0,
    };
    score_shape + score_match
}

fn strategy_2(inputs: Vec<&str>) -> u32 {
    let round_aim = match inputs[1] {
        "X" => 0, // lose
        "Y" => 3, // draw
        "Z" => 6, // win
        _ => 0,
    };

    let score_match = match inputs[0] {
        // rock
        "A" => match inputs[1] {
            "X" => 3, // scissors
            "Y" => 1, // rock
            "Z" => 2, // paper
            _ => 0,
        },
        // paper
        "B" => match inputs[1] {
            "X" => 1, // rock
            "Y" => 2, // paper
            "Z" => 3, // scissors
            _ => 0,
        },
        // scissors
        "C" => match inputs[1] {
            "X" => 2, // paper
            "Y" => 3, // scissors
            "Z" => 1, // rock
            _ => 0,
        },
        _ => 0,
    };
    round_aim + score_match
}

fn main() {
    let input = include_str!("../../inputs/2.txt");
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let inputs: Vec<&str> = line.split_whitespace().collect();
        part1.add_assign(strategy_1(inputs.clone()));
        part2.add_assign(strategy_2(inputs.clone()));
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

/*
A Y
B X
C Z
 */
