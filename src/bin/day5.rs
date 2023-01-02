use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::DerefMut;

#[derive(Debug)]
struct Crate {
    id: char,
}

impl Crate {
    pub fn new(id: char) -> Self {
        Crate { id: id }
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    pub fn new(crates: Vec<Crate>) -> Self {
        Self { crates }
    }

    pub fn move_crates(&mut self, number: usize, dest: &mut Stack) {
        let mut crates = self.crates.split_off(self.crates.len() - number);
        dest.crates.append(&mut crates);
    }
}

#[derive(Debug)]
struct Command {
    number_of_crates: usize,
    from: usize,
    to: usize,
}
static COMMAND_PARSE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

impl Command {
    pub fn parse(cmd_str: &String) -> Self {
        let caps = COMMAND_PARSE.captures(&cmd_str).unwrap();
        Self {
            number_of_crates: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }
}
fn execute_commands(
    commands: &Vec<Command>,
    stacks: Vec<RefCell<Stack>>,
    move_multiple_at_once: bool,
) -> Vec<RefCell<Stack>> {
    commands.iter().for_each(|command| {
        let mut to = stacks.get(command.to - 1).unwrap().borrow_mut();
        let mut from = stacks.get(command.from - 1).unwrap().borrow_mut();

        if !move_multiple_at_once {
            for _i in 1..command.number_of_crates + 1 {
                from.move_crates(1, to.deref_mut());
            }
        } else {
            from.move_crates(command.number_of_crates, to.deref_mut());
        }
    });
    stacks
}

fn build_stack(crates_lines: &Vec<String>) -> Vec<RefCell<Stack>> {
    let mut stacks: Vec<RefCell<Stack>> = Vec::new();
    let mut parsed_crate_lines: VecDeque<String> = crates_lines
        .iter()
        .rev()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let number_of_crates = (line.len() + 1) / 4;
            let mut values = String::new();
            for index in 1..number_of_crates + 1 {
                let char_index = 1 + (index - 1) * 4;
                let char_at_index = line.chars().nth(char_index).unwrap();
                values.push(char_at_index);
            }
            values
        })
        .collect();

    let first_line = parsed_crate_lines.pop_front().unwrap();
    for x in 0..first_line.len() {
        let mut crates: Vec<Crate> = Vec::new();
        for y in 0..parsed_crate_lines.len() {
            let id = parsed_crate_lines[y].chars().nth(x).unwrap();
            if id.to_string().trim().is_empty() {
                break;
            }
            crates.push(Crate::new(id));
        }
        stacks.push(RefCell::new(Stack::new(crates)));
    }
    stacks
}

fn main() {
    let input = include_str!("../../inputs/5.txt");

    let mut crates_lines: Vec<String> = Vec::new();
    let mut move_commands_lines: Vec<String> = Vec::new();

    input.lines().for_each(|l| {
        if l.starts_with("move") {
            move_commands_lines.push(l.to_string())
        } else {
            crates_lines.push(l.to_string())
        }
    });

    let commands: Vec<Command> = move_commands_lines.iter().map(Command::parse).collect();

    let part1 = execute_commands(&commands, build_stack(&crates_lines), false);
    let part2 = execute_commands(&commands, build_stack(&crates_lines), true);

    print!("part1: ");
    part1.iter().for_each(|stack| {
        print!("{}", stack.borrow().crates.last().unwrap().id);
    });
    print!("\n");

    print!("part2: ");
    part2.iter().for_each(|stack| {
        print!("{}", stack.borrow().crates.last().unwrap().id);
    });
    print!("\n");
}

/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
 */
