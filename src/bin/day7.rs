use core::str;
use path_absolutize::*;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

struct Directory {
    dirs: HashMap<PathBuf, usize>,
}

impl Directory {
    fn new() -> Self {
        return Self {
            dirs: HashMap::new(),
        };
    }

    pub fn parse_input(input: &str) -> Option<Self> {
        let terminal_commands: Vec<&str> = input
            .split("$")
            .filter(|l| !l.is_empty())
            .map(|l| l.trim())
            .collect();

        let mut result = Self::new();

        let mut current_directory = Path::new("/").to_path_buf();

        for terminal_command in terminal_commands {
            let mut tokens: VecDeque<&str> =
                terminal_command.split(|x| x == ' ' || x == '\n').collect();
            if let Some(cmd) = tokens.pop_front() {
                match cmd {
                    "cd" => {
                        let dir = tokens.pop_front()?;
                        current_directory = match dir {
                            "/" => current_directory.clone(),
                            _ => current_directory.join(dir),
                        };
                        current_directory = current_directory.absolutize().unwrap().to_path_buf();
                    }
                    "ls" => {
                        // populate directory
                        while !tokens.is_empty() {
                            let token = tokens.pop_front()?;
                            match token {
                                "dir" => {
                                    let name = tokens.pop_front()?;
                                    result.dirs.insert(current_directory.join(name), 0);
                                }
                                _ => {
                                    // file
                                    let _ = tokens.pop_front().unwrap();

                                    let directory_total = token.parse::<usize>().unwrap()
                                        + result.dirs.get(&current_directory).unwrap_or(&0);
                                    result
                                        .dirs
                                        .insert(current_directory.clone(), directory_total);
                                }
                            }
                        }
                    }
                    _ => println!("unknown cmd [{}]", cmd),
                };
            }
        }

        return Some(result);
    }

    pub fn total_size(&self, dir: &str) -> usize {
        self.dirs
            .keys()
            .filter(|path| path.starts_with(dir))
            .map(|path| *self.dirs.get(path).unwrap())
            .reduce(|acc, v| acc + v)
            .unwrap_or(0)
    }

    pub fn part1(&self) -> Option<usize> {
        self.dirs
            .keys()
            .map(|path| {
                let dir = path.to_str().unwrap();
                self.total_size(dir)
            })
            .filter(|size| size < &100000)
            .reduce(|acc, s| acc + s)
    }

    pub fn part2(&self) -> Option<usize> {
        let total_space: usize = 70000000;
        let required_space: usize = 30000000;

        let total_used_space = self.total_size("/");
        let remaining_space = total_space - total_used_space;
        if remaining_space > required_space {
            return None;
        }

        let mut possible_dirs = self
            .dirs
            .keys()
            .map(|p| (self.total_size(p.to_str().unwrap()), p))
            .filter(|pair| remaining_space + pair.0 >= required_space)
            .collect::<Vec<_>>();
        possible_dirs.sort_by(|a, b| a.0.cmp(&b.0));
        possible_dirs.truncate(1);
        if possible_dirs.is_empty() {
            None
        } else {
            let dir = possible_dirs.pop().unwrap();
            Some(dir.0)
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/7.txt");

    let file_directory = Directory::parse_input(input).unwrap();

    println!("part1: {}", file_directory.part1().unwrap());
    println!("part2: {}", file_directory.part2().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let file_directory = Directory::parse_input(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        )
        .unwrap();

        assert_eq!(file_directory.total_size("/d/"), 24933642);
        assert_eq!(file_directory.total_size("/a/e/"), 584);
        assert_eq!(file_directory.total_size("/a/"), 94853);
        assert_eq!(file_directory.total_size("/"), 48381165);

        assert_eq!(file_directory.part1().unwrap(), 95437);
        assert_eq!(file_directory.part2().unwrap(), 24933642);
    }
}
