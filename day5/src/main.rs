use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::{Deref, DerefMut};

use regex::Regex;

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

    pub fn drop(&mut self, c: Crate) {
        self.crates.push(c)
    }

    pub fn pick_up(&mut self) -> Option<Crate> {
        self.crates.pop()
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

fn main() -> io::Result<()> {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);
    let mut crates_lines: Vec<String> = Vec::new();
    let mut move_commands_lines: Vec<String> = Vec::new();

    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        if l.starts_with("move") {
            move_commands_lines.push(l.clone())
        } else {
            crates_lines.push(l.clone())
        }
    });

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

    // println!("stacks: {:?}", stacks);

    let commands: Vec<Command> = move_commands_lines.iter().map(Command::parse).collect();

    println!("commands = {:?}", commands);

    commands.iter().for_each(|command| {
        let mut to = stacks.get(command.to - 1).unwrap().borrow_mut();
        let mut from = stacks.get(command.from - 1).unwrap().borrow_mut();

        // 1st task
        // for _i in 1..command.number_of_crates + 1 {
        //     println!("Moving a crate from {} to {}", command.from, command.to);
        //     from.move_crates(1, to.deref_mut());
        // }

        // 2nd Task
        println!("Moving a crate from {} to {}", command.from, command.to);
        from.move_crates(command.number_of_crates, to.deref_mut());
    });

    stacks.iter().for_each(|stack| {
        print!("{}", stack.borrow().crates.last().unwrap().id);
    });

    Ok(())
}
