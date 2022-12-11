// Advent of Code 2022 - Day 11
type Item = u128;

#[derive(Debug)]
enum Operation {
    Add(u128),
    Multiply(u128),
    Square,
}

#[derive(Debug)]
enum Condition {
    DivisibleBy(u128),
}
#[derive(Debug)]
struct Monkey {
    items_inspected: u128,
    items: Vec<Item>,
    operation: Operation,
    condition: Condition,
    next_monkey_if_true: u128,
    next_monkey_if_false: u128,
}

fn parse_monkey(str: &str) -> Monkey {
    let rows = str.split("\n").collect::<Vec<&str>>();

    let items = parse_starting_items(rows[1]);
    let operation = parse_operation(rows[2]);

    let (condition, if_true, if_false) = parse_test(rows[3..].to_vec());

    return Monkey {
        items_inspected: 0,
        items,
        operation,
        condition,
        next_monkey_if_true: if_true,
        next_monkey_if_false: if_false,
    };
}

fn parse_test(rows: Vec<&str>) -> (Condition, u128, u128) {
    let condition: Condition = match rows[0]
        .trim()
        .strip_prefix("Test: divisible by ")
        .expect("should start with")
    {
        num => Condition::DivisibleBy(num.parse().expect("should be number")),
    };

    let if_true: u128 = rows[1]
        .trim()
        .strip_prefix("If true: throw to monkey ")
        .expect("should start with")
        .parse()
        .expect("should be number");
    let if_false: u128 = rows[2]
        .trim()
        .strip_prefix("If false: throw to monkey ")
        .expect("should start with")
        .parse()
        .expect("should be number");

    return (condition, if_true, if_false);
}

fn parse_starting_items(s: &str) -> Vec<Item> {
    return s
        .strip_prefix("  Starting items: ")
        .expect("should start with starting items")
        .split(", ")
        .map(|s| s.trim().parse().expect("should be number"))
        .collect();
}

fn parse_operation(s: &str) -> Operation {
    return match s
        .strip_prefix("  Operation: new = old ")
        .expect("should start with operation")
        .split_once(" ")
        .expect("should have space")
    {
        ("*", "old") => Operation::Square,
        ("+", num) => Operation::Add(num.parse().expect("should be number")),
        ("*", num) => Operation::Multiply(num.parse().expect("should be number")),
        _ => panic!("Unknown operation"),
    };
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");

    let monkeys_strings = input.split("\n\n");

    let monkeys: Vec<Monkey> = monkeys_strings.map(parse_monkey).collect();

    match std::env::var("part").unwrap_or_default().as_str() {
        "part2" => print!("{}", part_two(monkeys)),
        _ => println!("{}", part_one(monkeys)),
    };
}

fn part_two(mut monkeys: Vec<Monkey>) -> u128 {
    let greatest_common_denominator = monkeys
        .iter()
        .map(|monkey| match monkey.condition {
            Condition::DivisibleBy(num) => num,
        })
        .fold(1, |acc, num| acc * num);

    for _ in 0..10000 {
        for current_index in 0..monkeys.len() {
            let items_len = monkeys[current_index].items.len();
            for index in 0..items_len {
                let item = monkeys[current_index].items[index];
                monkeys[current_index].items_inspected += 1;
                let item_picked_up = match monkeys[current_index].operation {
                    Operation::Add(num) => item + num,
                    Operation::Multiply(num) => item * num,
                    Operation::Square => item * item,
                };

                let item_after_relif = item_picked_up % greatest_common_denominator;

                // trow to next monkey
                let fulfills_condition: bool = match monkeys[current_index].condition {
                    Condition::DivisibleBy(num) => item_after_relif % num == 0,
                };

                let next_monkey_index = if fulfills_condition {
                    monkeys[current_index].next_monkey_if_true
                } else {
                    monkeys[current_index].next_monkey_if_false
                };

                monkeys[next_monkey_index as usize]
                    .items
                    .push(item_after_relif);
            }
            monkeys[current_index].items.clear();
        }
    }
    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    return monkeys[..2]
        .iter()
        .map(|m| m.items_inspected as u128)
        .product();
}

fn part_one(mut monkeys: Vec<Monkey>) -> u128 {
    for _ in 0..20 {
        for current_index in 0..monkeys.len() {
            let items_len = monkeys[current_index].items.len();
            for index in 0..items_len {
                let item = monkeys[current_index].items[index];
                monkeys[current_index].items_inspected += 1;
                let item_picked_up = match monkeys[current_index].operation {
                    Operation::Add(num) => item + num,
                    Operation::Multiply(num) => item * num,
                    Operation::Square => item * item,
                };

                let item_after_relif = item_picked_up / 3;
                let fulfills_condition: bool = match monkeys[current_index].condition {
                    Condition::DivisibleBy(num) => item_after_relif % num == 0,
                };

                let next_monkey_index = if fulfills_condition {
                    monkeys[current_index].next_monkey_if_true
                } else {
                    monkeys[current_index].next_monkey_if_false
                };

                monkeys[next_monkey_index as usize]
                    .items
                    .push(item_after_relif);
            }
            monkeys[current_index].items.clear();
        }
    }
    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    return monkeys[..2]
        .iter()
        .map(|m| m.items_inspected as u128)
        .product();
}
