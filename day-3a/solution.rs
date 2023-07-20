use std::fs::read_to_string;
use std::collections::HashMap;

fn get_priority(c: char) -> u32 {
    let val = c as u32;
    if val > 'a' as u32 {
        return val - 'a' as u32 + 1;
    }
    return val - 'A' as u32 + 27;
}

fn get_line_val(line: &str) -> u32 {
    let mut letters: HashMap<char, bool> = HashMap::with_capacity(52);
    let mid_point: usize = line.len() >> 1;
    let mut chars = line.chars();
    
    for _ in 0..mid_point {
        if let Some(c) = chars.next() {
            letters.insert(c, true);
        }
    }

    for _ in mid_point..line.len() {
        if let Some(c) = chars.next() {
            if let Some(_) = letters.get(&c) {
                // println!("{}", get_priority(c));
                return get_priority(c);
            }
        }
    }

    return 0;
}

fn main() -> std::io::Result<()> {
    let total: u32 = read_to_string("day-3a/input.txt")
        .unwrap()
        .lines()
        .map(get_line_val)
        .sum();

    println!("{}", total);
    Ok(())
}
