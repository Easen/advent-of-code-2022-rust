use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line?;
        let inputs: Vec<&str> = line.split_whitespace().collect();

        let score_shape = match inputs[1] {
            "X" => 1, // rock
            "Y" => 2, // paper
            "Z" => 3, // scissors
            _ => 0,
        };
        println!("score_shape: {}", score_shape);

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
        println!("score_match: {}", score_match);
        let round_total = score_shape + score_match;
        println!("round total: {}", round_total);

        total_score = total_score + round_total;
    }
    println!("total score: {}", total_score);

    Ok(())
}
