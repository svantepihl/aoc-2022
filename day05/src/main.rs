fn parse_input<'a>(
    input: &'a str,
) -> (
    Vec<Vec<char>>,
    impl Iterator<Item = (usize, usize, usize)> + 'a,
) {
    let (start, moves) = input.split_once("\n\n").unwrap();
    let start = start
        .lines()
        .rev()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|&(i, _)| i % 4 == 1)
                .map(|(_, e)| e)
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut columns = vec![vec![]; start[0].len()];
    for row in start {
        for (i, c) in row.iter().enumerate() {
            if !c.is_whitespace() && !c.is_digit(10) {
                columns[i].push(*c);
            }
        }
    }

    let moves = moves.lines().map(|line| {
        let row: Vec<&str> = line.split_whitespace().collect();
        (
            row[1].parse::<usize>().unwrap(),
            row[3].parse::<usize>().unwrap(),
            row[5].parse::<usize>().unwrap(),
        )
    });
    (columns, moves)
}

fn mover_p1(state: &mut Vec<Vec<char>>, moves: impl Iterator<Item = (usize, usize, usize)>) {
    moves.for_each(|(count, from, to)| {
        for _ in 0..count {
            let moving_crate = state[from - 1].pop().unwrap();
            state[to - 1].push(moving_crate);
        }
    });
}

fn mover_p2(state: &mut Vec<Vec<char>>, moves: impl Iterator<Item = (usize, usize, usize)>) {
    let mut buffer: Vec<char> = vec![];
    moves.for_each(|(count, from, to)| {
        for _ in 0..count {
            let moving_crate = state[from - 1].pop().unwrap();
            buffer.push(moving_crate);
        }

        while let Some(c) = buffer.pop() {
            state[to - 1].push(c);
        }
    });
}

fn main() {
    let binding = std::env::var("part").unwrap_or_else(|_e| "part1".to_string());
    let part = binding.as_str();

    let input = std::fs::read_to_string("input.txt").unwrap();
    let (start, moves) = parse_input(&input);

    let mut state = start;

    match part {
        "part2" => mover_p2(&mut state, moves),
        _ => mover_p1(&mut state, moves),
    }

    state
        .iter()
        .for_each(|row| print!("{}", row.last().unwrap()));
}
