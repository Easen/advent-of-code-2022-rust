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

    pub fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);
    let mut total_overlap = 0;
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(",").collect();
        let range1 = Range::new(parts[0]);
        let range2 = Range::new(parts[1]);

        if range1.contains(&range2) || range2.contains(&range1) {
            total_overlap = total_overlap + 1;
        }

        println!(
            "line = {}, range1 = {:?}, range2 = {:?}, range 1 contains range 2 = {:?}, range 2 contains range 1 = {:?}",
            line, range1, range2, range1.contains(&range2), range2.contains(&range1)
        )
    }
    println!("total_overlap: {}", total_overlap);

    Ok(())
}
