type Tree = u32;
type Forest = Vec<Vec<Tree>>;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");
    let forest: Forest = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should be digit"))
                .collect()
        })
        .collect();

    match std::env::var("part").unwrap_or_default().as_str() {
        "part2" => print!("{}", part_two(&forest)),
        _ => println!("{}", part_one(&forest)),
    };
}

fn part_two(forest: &Forest) -> u32 {
    let mut heighest_score = 0;
    for (y, row) in forest.iter().enumerate() {
        for (x, current_tree) in row.iter().enumerate() {
            if y == 0 || x == 0 || y == forest.len() - 1 || x == forest.len() - 1 {
                continue;
            }

            let viewing_distance_right = view_right(x, y, current_tree, forest);
            let viewing_distance_left = view_left(x, y, current_tree, forest);
            let viewing_distance_up = view_up(x, y, current_tree, forest);
            let viewing_distance_down = view_down(x, y, current_tree, forest);

            let scenic_score = viewing_distance_right
                * viewing_distance_left
                * viewing_distance_up
                * viewing_distance_down;

            if scenic_score > heighest_score {
                heighest_score = scenic_score;
            }
        }
    }
    return heighest_score;
}

fn part_one(forest: &Forest) -> i32 {
    let mut tree_count = 0;
    for (y, row) in forest.iter().enumerate() {
        for (x, current_tree) in row.iter().enumerate() {
            if y == 0 || x == 0 || y == forest.len() - 1 || x == forest.len() - 1 {
                tree_count += 1;
                continue;
            }

            if check_left(x, y, current_tree, forest)
                || check_right(x, y, current_tree, forest)
                || check_up(x, y, current_tree, forest)
                || check_down(x, y, current_tree, forest)
            {
                tree_count += 1;
                continue;
            }
        }
    }
    tree_count
}
fn view_down(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> u32 {
    let trees_down = &forest[y + 1..];
    return match trees_down.iter().position(|row| &row[x] >= current_tree) {
        Some(index) => index as u32 + 1,
        None => trees_down.len() as u32,
    };
}

fn view_up(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> u32 {
    let trees_up = &forest[..y];
    return match trees_up
        .iter()
        .rev()
        .position(|row| &row[x] >= current_tree)
    {
        Some(index) => index as u32 + 1,
        None => trees_up.len() as u32,
    };
}

fn view_left(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> u32 {
    let trees_left = &forest[y][..x];
    return match trees_left
        .iter()
        .rev()
        .position(|tree| tree >= current_tree)
    {
        Some(index) => index as u32 + 1,
        None => trees_left.len() as u32,
    };
}

fn view_right(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> u32 {
    let trees_right = &forest[y][x + 1..];
    return match trees_right.iter().position(|tree| tree >= current_tree) {
        Some(index) => index as u32 + 1,
        None => trees_right.len() as u32,
    };
}

fn check_up(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> bool {
    let rows_above = &forest[..y];
    return rows_above.iter().all(|row| &row[x] < current_tree);
}

fn check_left(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> bool {
    let tree_to_left = &forest[y][..x];
    return tree_to_left.iter().all(|tree| tree < current_tree);
}

fn check_right(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> bool {
    let tree_to_right = &forest[y][x + 1..];
    return tree_to_right.iter().all(|tree| tree < current_tree);
}

fn check_down(x: usize, y: usize, current_tree: &u32, forest: &Forest) -> bool {
    let rows_below = &forest[y + 1..];
    return rows_below.iter().all(|row| &row[x] < current_tree);
}
