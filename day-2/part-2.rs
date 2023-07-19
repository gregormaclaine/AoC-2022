use std::fs::read_to_string;
use std::collections::HashMap;

fn get_score(shapes: &HashMap<char, i32>, elf: char, you: char) -> i32 {
    let shape_val: i32 = *shapes.get(&elf).unwrap();
    match you {
        'X' => (shape_val + 2) % 3 + 1,
        'Y' => shape_val + 4,
        'Z' => (shape_val + 1) % 3 + 7,
         _  => 0,
    }
}

fn main() -> std::io::Result<()> {
    let shapes: HashMap<char, i32> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2)
    ]);

    let total: i32 = read_to_string("day-2/input.txt")
        .unwrap()
        .lines()
        .map(|x| get_score(&shapes, x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap()))
        .sum();

    println!("{}", total);
    Ok(())
}