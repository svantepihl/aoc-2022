// Advent of Code 2022 - Day 10

use std::{collections::HashMap, vec};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}
impl Instruction {
    fn parse(str: &str) -> Instruction {
        let mut parts = str.split_whitespace();
        let command = parts.next().expect("No command");
        let value = parts.next();

        match command {
            "addx" => Instruction::Addx(value.unwrap().parse().expect("Invalid value")),
            _ => Instruction::Noop,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");

    let instructions = input.lines().map(Instruction::parse);

    let mut cycles: HashMap<i32, i32> = HashMap::new();

    let mut cycle = 1;
    let mut x = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Addx(value) => {
                cycles.insert(cycle, x);
                cycle += 1;
                cycles.insert(cycle, x);
                cycle += 1;
                x += value;
            }
            Instruction::Noop => {
                cycles.insert(cycle, x);
                cycle += 1;
            }
        }
    }

    match std::env::var("part").unwrap_or_default().as_str() {
        "part2" => print!("{}", part_two(cycles)),
        _ => println!("{}", part_one(cycles)),
    };
    //part_one(cycles);
}

fn part_two(cycles: HashMap<i32, i32>) -> String {
    let mut result: Vec<char> = vec![];
    for cycle in 1..=240 {
        let spirte_middle = cycles.get(&cycle).expect("we should have this");
        let sprite = [spirte_middle - 1, *spirte_middle, spirte_middle + 1];

        result.push(if sprite.contains(&((cycle - 1) % 40)) {
            '#'
        } else {
            ' '
        });
    }

    return result
        .chunks(40)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn part_one(cycles: HashMap<i32, i32>) -> i32 {
    let key_cycle = vec![20, 60, 100, 140, 180, 220];
    return key_cycle
        .iter()
        .map(|cycle| cycles.get(cycle).unwrap() * cycle)
        .sum();
}
