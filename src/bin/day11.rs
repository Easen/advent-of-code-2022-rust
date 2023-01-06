#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Multiple(usize),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible_by: usize,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    pub fn from_str(input: &str) -> Self {
        let tokens: Vec<&str> = input
            .lines()
            .skip(1)
            .map(|l| l.split(":").nth(1))
            .map(|t| t.unwrap())
            .collect();
        let items = tokens
            .get(0)
            .unwrap()
            .split(",")
            .map(|t| t.trim().parse().unwrap())
            .collect::<Vec<_>>();

        let operation = tokens.get(1).unwrap().trim();
        let operation = if operation.contains("old * old") {
            Operation::Square
        } else {
            let val = operation
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            if operation.contains("+") {
                Operation::Add(val)
            } else {
                Operation::Multiple(val)
            }
        };

        let test_divisible_by = tokens
            .get(2)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test_true = tokens
            .get(3)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let test_false = tokens
            .get(4)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            items,
            operation,
            test_divisible_by,
            test_true,
            test_false,
        }
    }
}

fn part1(monkeys: &mut Vec<Monkey>) -> usize {
    play_game(monkeys, 20, |w| w / 3)
}

fn part2(monkeys: &mut Vec<Monkey>) -> usize {
    let all_worries: usize = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product();
    play_game(monkeys, 10000, |w| w % all_worries)
}

fn play_game(monkeys: &mut Vec<Monkey>, rounds: i32, worry_fn: impl Fn(usize) -> usize) -> usize {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            monkey.items.into_iter().for_each(|item| {
                let mut worry_level = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiple(x) => item * x,
                    Operation::Square => item * item,
                };
                worry_level = worry_fn(worry_level);
                let monkey_index = if worry_level % monkey.test_divisible_by == 0 {
                    monkey.test_true
                } else {
                    monkey.test_false
                };
                monkeys[monkey_index].items.push(worry_level);
                inspections[i] += 1;
            });
            monkeys[i].items.clear();
        }
    }
    inspections.sort();
    let monkey_business = inspections.pop().unwrap() * inspections.pop().unwrap();
    return monkey_business;
}

fn main() {
    let input = include_str!("../../inputs/11.txt");
    let monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from_str).collect();
    println!("part1: {}", part1(&mut monkeys.clone()));
    println!("part2: {}", part2(&mut monkeys.clone()));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from_str).collect();
        assert_eq!(part1(&mut monkeys), 10605);
    }
    #[test]
    fn test_example_part2() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from_str).collect();
        assert_eq!(part2(&mut monkeys), 2713310158);
    }
}
