use std::collections::HashMap;
fn main() {
    let input = include_str!("../../inputs/1.txt");

    let mut totals: HashMap<i32, i32> = HashMap::new();

    let mut index = 1;
    for line in input.lines() {
        if line.eq("") {
            index = index + 1;
            continue;
        }

        let current_cal: i32 = line.parse().unwrap_or(0);
        match totals.get(&index) {
            Some(x) => totals.insert(index, x + current_cal),
            None => totals.insert(index, current_cal),
        };
    }

    let mut sorted: Vec<_> = totals.iter().collect();
    sorted.sort_by_key(|a| !a.1);

    let values: Vec<i32> = sorted.iter().map(|x| *x.1).collect();
    println!("part 1: {:?}", values.get(0).unwrap());

    let part2: i32 = values.iter().take(3).sum();

    println!("part 2: {}", part2);
}
