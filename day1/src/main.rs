use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::AddAssign;

fn main() -> io::Result<()> {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    let mut totals: HashMap<i32, i32> = HashMap::new();

    let mut index = 1;
    for line in reader.lines() {
        let line = line?;
        if line.eq("") {
            index = index + 1;
            println!("increment index {}", index);
            continue;
        }

        let current_cal: i32 = line.parse().unwrap_or(0);
        match totals.get(&index) {
            Some(x) => totals.insert(index, x + current_cal),
            None => totals.insert(index, current_cal),
        };
        println!(
            "index: {}, current_cal: {}, total: {}",
            index,
            current_cal,
            totals.get(&index).unwrap()
        );
    }

    let mut sorted: Vec<_> = totals.iter().collect();
    sorted.sort_by_key(|a| !a.1);
    let mut total = 0;
    for i in 0..3 {
        let entry = sorted.iter().nth(i).unwrap();
        println!("Top {}: {} {}", i + 1, entry.0, entry.1);
        total.add_assign(entry.1);
    }

    println!("sum of top 3: {}", total);

    Ok(())
}
