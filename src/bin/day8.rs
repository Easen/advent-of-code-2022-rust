use std::ops::AddAssign;

struct Grid {
    visible_trees: usize,
    scenic_scores: Vec<usize>,
}

impl Grid {
    fn calculate_visibility(trees: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
        let tree = &trees[row][col];

        let left = &trees[row][0..col].iter().all(|t| t < tree);
        let right = &trees[row][col + 1..trees[row].len()]
            .iter()
            .all(|t| t < tree);
        let top = &trees[0..row].iter().all(|t| &t[col] < tree);
        let bottom = &trees[row + 1..trees.len()].iter().all(|t| &t[col] < tree);

        (*left || *right || *top || *bottom) as usize
    }

    pub fn using_map(map: &str) -> Self {
        let trees = map
            .lines()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut visible_trees = trees.first().unwrap_or(&(vec![] as Vec<u8>)).len()
            + (trees.len() - 2) * 2
            + trees.last().unwrap_or(&(vec![] as Vec<u8>)).len();

        let mut scenic_scores = Vec::new();

        for row in 1..trees.get(1).unwrap_or(&(vec![] as Vec<u8>)).len() - 1 {
            for col in 1..trees.get(row).unwrap_or(&(vec![] as Vec<u8>)).len() - 1 {
                visible_trees.add_assign(Grid::calculate_visibility(&trees, row, col));

                let tree = trees[row][col];
                let left = Grid::calculate_score(trees[row][0..col].iter().rev(), tree, col);
                let right = Grid::calculate_score(
                    trees[row][col + 1..trees[row].len()].iter(),
                    tree,
                    trees[row].len() - col - 1,
                );
                let top = Grid::calculate_score(
                    trees[0..row].iter().map(|row| &row[col]).rev(),
                    tree,
                    row,
                );
                let bottom = Grid::calculate_score(
                    trees[row + 1..trees.len()].iter().map(|row| &row[col]),
                    tree,
                    trees.len() - row - 1,
                );

                let total = left * right * top * bottom;
                scenic_scores.push(total)
            }
        }

        Self {
            visible_trees,
            scenic_scores,
        }
    }

    pub fn part1(&self) -> Option<usize> {
        if self.visible_trees > 0 {
            Some(self.visible_trees)
        } else {
            None
        }
    }
    pub fn part2(&self) -> Option<usize> {
        if self.scenic_scores.is_empty() {
            return None;
        }
        let mut scores = self.scenic_scores.clone();
        scores.sort_by(|a, b| b.cmp(a));
        scores.truncate(1);
        return Some(scores[0]);
    }

    fn calculate_score<'a, I>(range: I, tree: u8, default: usize) -> usize
    where
        I: Iterator<Item = &'a u8>,
    {
        range
            .into_iter()
            .position(|&t| t >= tree)
            .map_or(default, |v| v + 1)
    }
}

fn main() {
    let input = include_str!("../../inputs/8.txt");

    let grid = Grid::using_map(input);

    println!("part1: {:?}", grid.part1());
    println!("part2: {:?}", grid.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = Grid::using_map(
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(grid.part1().unwrap(), 21);
        assert_eq!(grid.part2().unwrap(), 8);
    }
}
