use std::io::{BufRead, BufReader};

fn main() {
    match std::env::var("part")
        .unwrap_or_else(|_e| "part1".to_string())
        .as_str()
    {
        "part1" => {
            println!("Part 1");
            BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .map(|line| line.unwrap())
                .for_each(|line| println!("{}", line));
        }
        _ => {
            println!("Part 2");
        }
    };
}
