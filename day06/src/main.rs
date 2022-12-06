fn main() {
    // read file as bytes
    let input = include_bytes!("../input.txt");

    let marker_len = match std::env::var("part").unwrap_or_default().as_str() {
        "part2" => 14,
        _ => 4,
    };

    let mut i = 0;
    while i < input.len() - 4 {
        if is_unique(&input[i..i + marker_len]) {
            i += marker_len;
            break;
        } else {
            i += 1;
        }
    }
    println!("{}", i)
}

fn is_unique(slice: &[u8]) -> bool {
    let mut unique = true;
    for i in 0..slice.len() {
        for j in 0..slice.len() {
            if i != j && slice[i] == slice[j] {
                unique = false;
                break;
            }
        }
    }
    unique
}
