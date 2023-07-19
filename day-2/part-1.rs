use std::fs::read_to_string;
use std::collections::HashMap;

fn get_score(shapes: &HashMap<char, u32>, elf: char, you: char) -> u32 {
    let score: u32 = 1 + shapes.get(&you).unwrap();
    if *shapes.get(&elf).unwrap() == *shapes.get(&you).unwrap() {
        return score + 3;
    }
    if (*shapes.get(&elf).unwrap() + 1) % 3 == *shapes.get(&you).unwrap() {
        return score + 6;
    }
    return score;
}

fn main() -> std::io::Result<()> {
    let shapes: HashMap<char, u32> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('X', 0),
        ('Y', 1),
        ('Z', 2)
    ]);

    let total: u32 = read_to_string("day-2/input.txt")
        .unwrap()
        .lines()
        .map(|x| get_score(&shapes, x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap()))
        .sum();

    println!("{}", total);
    Ok(())
}