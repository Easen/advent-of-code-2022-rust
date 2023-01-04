use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone)]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }

    fn calculate_move(&self, instruction: Instruction) -> Self {
        let mut next_move = self.clone();
        match instruction.direction {
            Direction::Right => next_move.head.x = next_move.head.x + 1,
            Direction::Left => next_move.head.x = next_move.head.x - 1,
            Direction::Up => next_move.head.y = next_move.head.y + 1,
            Direction::Down => next_move.head.y = next_move.head.y - 1,
        };

        let delta = Point {
            x: next_move.head.x - next_move.tail.x,
            y: next_move.head.y - next_move.tail.y,
        };

        if delta.x.abs() > 1 || delta.y.abs() > 1 {
            // move
            if delta.x.abs() == 2 && delta.y.abs() == 0 {
                // x axis only
                next_move.tail.x = next_move.tail.x + delta.x / 2;
            } else if delta.x.abs() == 0 && delta.y.abs() > 1 {
                // y axis
                next_move.tail.y = next_move.tail.y + delta.y / 2;
            } else {
                // both
                next_move.tail.x = next_move.tail.x + if delta.x > 0 { 1 } else { -1 };
                next_move.tail.y = next_move.tail.y + if delta.y > 0 { 1 } else { -1 };
            }
        }

        return next_move;
    }

    fn calculate_moves(&self, instruction: Instruction) -> Vec<Rope> {
        if let Some(next_instruction) = instruction.next() {
            let new_rope = self.calculate_move(instruction);

            let mut result = vec![new_rope];
            result.extend(result[0].calculate_moves(next_instruction));
            return result;
        }
        return vec![];
    }
}

#[derive(Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn from_str(direction: &str) -> Option<Self> {
        match direction {
            "R" => Some(Self::Right),
            "L" => Some(Self::Left),
            "U" => Some(Self::Up),
            "D" => Some(Self::Down),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Instruction {
    direction: Direction,
    steps: u8,
}

impl Instruction {
    pub fn from_str(instruction: &str) -> Option<Self> {
        let parts: Vec<&str> = instruction.splitn(2, " ").collect();

        if parts.len() != 2 {
            return None;
        }

        if let Some(direction) = Direction::from_str(parts[0]) {
            if let Some(steps) = parts.get(1).unwrap().parse::<u8>().ok() {
                return Some(Self { direction, steps });
            }
        }
        None
    }

    fn next(&self) -> Option<Self> {
        if self.steps == 0 {
            None
        } else {
            Some(Self {
                direction: self.direction.clone(),
                steps: self.steps - 1,
            })
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/9.txt");

    let rope = Rope::new();

    let calculated_moves: Vec<Rope> = input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .fold(vec![rope], |mut acc, instruction| {
            let next_moves = acc.last().unwrap().calculate_moves(instruction);
            acc.extend(next_moves);
            return acc;
        });

    let part1: HashSet<Point> = calculated_moves.iter().map(|m| m.tail).collect();
    println!("part1: {}", part1.len());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_example() {
        let rope = Rope::new();

        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let calculated_moves: Vec<Rope> = input
            .lines()
            .map(|l| Instruction::from_str(l).unwrap())
            .fold(vec![rope], |mut acc, instruction| {
                let next_moves = acc.last().unwrap().calculate_moves(instruction);
                acc.extend(next_moves);
                return acc;
            });

        let part1: HashSet<Point> = calculated_moves.iter().map(|m| m.tail).collect();
        assert_eq!(part1.len(), 13); //?
    }
}
