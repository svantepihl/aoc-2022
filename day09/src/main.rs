use std::{collections::HashSet, vec};

enum Direction {
    U,
    D,
    L,
    R,
}

struct Command {
    direction: Direction,
    distance: i32,
}

fn parse_command(s: &str) -> Command {
    let (direction, distance) = s.split_once(" ").expect("Invalid command");
    let direction = match direction {
        "U" => Direction::U,
        "D" => Direction::D,
        "L" => Direction::L,
        "R" => Direction::R,
        _ => panic!("Unknown direction"),
    };
    let distance = distance.parse().expect("Invalid distance");
    Command {
        direction,
        distance,
    }
}

fn part_one(commands: impl Iterator<Item = Command>) -> usize {
    let start = (0, 0);
    let mut head = start;
    let mut tail = start;
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(tail);

    for command in commands {
        for _ in 0..command.distance {
            match command.direction {
                Direction::U => head.1 += 1,
                Direction::D => head.1 -= 1,
                Direction::L => head.0 -= 1,
                Direction::R => head.0 += 1,
            }
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                match command.direction {
                    Direction::U => {
                        tail.1 += 1;
                        tail.0 = head.0;
                    }
                    Direction::D => {
                        tail.1 -= 1;
                        tail.0 = head.0
                    }
                    Direction::L => {
                        tail.0 -= 1;
                        tail.1 = head.1
                    }
                    Direction::R => {
                        tail.0 += 1;
                        tail.1 = head.1
                    }
                }
                tail_visited.insert(tail);
            }
        }
    }

    return tail_visited.len();
}

fn part_two(commands: impl Iterator<Item = Command>) -> usize {
    // fill rope with start
    let start = (0, 0);
    let mut rope = vec![start; 10];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(start);

    for command in commands {
        for _ in 0..command.distance {
            match command.direction {
                Direction::U => rope[0].1 += 1,
                Direction::D => rope[0].1 -= 1,
                Direction::L => rope[0].0 -= 1,
                Direction::R => rope[0].0 += 1,
            }

            // loop through and follow the head
            for index in 1..rope.len() {
                if (rope[index - 1].0 - rope[index].0).abs() > 1
                    || (rope[index - 1].1 - rope[index].1).abs() > 1
                {
                    // follow the previous
                    if rope[index - 1].0 != rope[index].0 {
                        if rope[index].0 > rope[index - 1].0 {
                            rope[index].0 -= 1;
                        } else {
                            rope[index].0 += 1;
                        }
                    }

                    if rope[index - 1].1 != rope[index].1 {
                        if rope[index].1 > rope[index - 1].1 {
                            rope[index].1 -= 1;
                        } else {
                            rope[index].1 += 1;
                        }
                    }
                }
            }
            tail_visited.insert(rope[9]);
        }
    }
    return tail_visited.len();
}
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");

    let commands = input.lines().map(|line| parse_command(line));

    match std::env::var("part").unwrap_or_default().as_str() {
        "part2" => print!("{}", part_two(commands)),
        _ => println!("{}", part_one(commands)),
    };
}
