use std::fs::read_to_string;
use std::collections::HashMap;

fn get_priority(c: char) -> u32 {
    let val = c as u32;
    if val > 'a' as u32 {
        return val - 'a' as u32 + 1;
    }
    return val - 'A' as u32 + 27;
}

fn get_group_val(line1: &str, line2: &str, line3: &str) -> u32 {
    let mut letters1: HashMap<char, bool> = HashMap::with_capacity(52);
    let mut letters2: HashMap<char, bool> = HashMap::with_capacity(52);
    
    for c in line1.chars() { letters1.insert(c, true); }
    for c in line2.chars() { letters2.insert(c, true); }

    for c in line3.chars() {
        if let Some(_) = letters1.get(&c) {
            if let Some(_) = letters2.get(&c) {
                // println!("{} {}", c, get_priority(c));
                return get_priority(c);
            }
        }
    }

    return 0;
}

fn main() -> std::io::Result<()> {
    let binding = read_to_string("day-3/input.txt").unwrap();
    let mut lines = binding.lines();

    let mut total: u32 = 0;
    while let Some(line) = lines.next() {
        total += get_group_val(line, lines.next().unwrap(), lines.next().unwrap());
    }

    println!("{}", total);
    Ok(())
}
