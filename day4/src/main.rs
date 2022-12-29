use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    pub fn new(range: &str) -> Self {
        let range_parts: Vec<u32> = range.split("-").map(|s| s.parse().unwrap()).collect();
        Self {
            lower: range_parts[0],
            upper: range_parts[1],
        }
    }

    pub fn contains_all(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains_all(other) || other.contains_all(self)
    }

    pub fn partial_overlap(&self, other: &Range) -> bool {
        self.upper >= other.lower && other.upper >= self.lower
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);
    let total_overlap = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(",")
                .map(Range::new)
                .collect::<Vec<Range>>()
        })
        .map(|pair| {
            (
                pair[0].overlaps(&pair[1]) as u32,
                pair[0].partial_overlap(&pair[1]) as u32,
            )
        })
        .map(|t| {
            println!("tuple {:?}", t);
            t
        })
        .reduce(|total, item| (total.0 + item.0, total.1 + item.1))
        .unwrap_or((0, 0));

    println!("total_overlap: {:?}", total_overlap);

    Ok(())
}
