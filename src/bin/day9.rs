use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}
#[derive(Clone)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let points: Vec<Point> = vec![Point::new(); length];
        Self { knots: points }
    }

    pub fn head(&self) -> Point {
        *self.knots.first().unwrap()
    }

    pub fn tail(&self) -> Point {
        *self.knots.last().unwrap()
    }

    fn calculate_move(&self, instruction: &Instruction) -> Self {
        let mut next_move = self.clone();
        let mut head = next_move.head().clone();
        match instruction.direction {
            Direction::Right => head.x = head.x + 1,
            Direction::Left => head.x = head.x - 1,
            Direction::Up => head.y = head.y + 1,
            Direction::Down => head.y = head.y - 1,
        };
        next_move.knots[0] = head;

        fn compute_knot(current: Point, previous: Point) -> Point {
            let delta = Point {
                x: previous.x - current.x,
                y: previous.y - current.y,
            };

            let mut result = current.clone();

            if delta.x.abs() > 1 || delta.y.abs() > 1 {
                // move
                if delta.x.abs() == 2 && delta.y.abs() == 0 {
                    // x axis only
                    result.x = result.x + delta.x / 2;
                } else if delta.x.abs() == 0 && delta.y.abs() > 1 {
                    // y axis
                    result.y = result.y + delta.y / 2;
                } else {
                    // both
                    result.x = result.x + if delta.x > 0 { 1 } else { -1 };
                    result.y = result.y + if delta.y > 0 { 1 } else { -1 };
                }
            }
            result
        }

        for i in 1..self.knots.len() {
            next_move.knots[i] = compute_knot(next_move.knots[i], next_move.knots[i - 1]);
        }

        return next_move;
    }

    fn calculate_moves(&self, instruction: &Instruction) -> Vec<Rope> {
        if let Some(next_instruction) = instruction.next() {
            let new_rope = self.calculate_move(&instruction);

            let mut result = vec![new_rope];
            result.extend(result[0].calculate_moves(&next_instruction));
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
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect();

    let rope = Rope::new(2);
    let part1_tail_positions: HashSet<Point> = instructions
        .iter()
        .fold(vec![rope], |mut acc, instruction| {
            let next_moves = acc.last().unwrap().calculate_moves(&instruction);
            acc.extend(next_moves);
            return acc;
        })
        .iter()
        .map(|m| m.tail())
        .collect();
    println!("part1: {}", part1_tail_positions.len());

    let rope = Rope::new(10);
    let part2_tail_positions: HashSet<Point> = instructions
        .iter()
        .fold(vec![rope], |mut acc, instruction| {
            let next_moves = acc.last().unwrap().calculate_moves(&instruction);
            acc.extend(next_moves);
            return acc;
        })
        .iter()
        .map(|m| m.tail())
        .collect();
    println!("part2: {}", part2_tail_positions.len());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_example_part1() {
        let rope = Rope::new(2);

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
                let next_moves = acc.last().unwrap().calculate_moves(&instruction);
                acc.extend(next_moves);
                return acc;
            });

        let part1: HashSet<Point> = calculated_moves.iter().map(|m| m.tail()).collect();
        assert_eq!(part1.len(), 13);
    }

    #[test]
    fn test_example_part2() {
        let rope = Rope::new(10);

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
                let next_moves = acc.last().unwrap().calculate_moves(&instruction);
                acc.extend(next_moves);
                return acc;
            });

        let part2: HashSet<Point> = calculated_moves.iter().map(|m| m.tail()).collect();
        assert_eq!(part2.len(), 1);
    }
}
