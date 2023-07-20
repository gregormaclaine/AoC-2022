use std::fs::read_to_string;

fn get_score(elf: char, you: char) -> u32 {
    let shape_val: u32 = elf as u32 - 'A' as u32;
    match you {
        'X' => (shape_val + 2) % 3 + 1,
        'Y' => shape_val + 4,
        'Z' => (shape_val + 1) % 3 + 7,
         _  => 0,
    }
}

fn main() -> std::io::Result<()> {
    let total: u32 = read_to_string("day-2a/input.txt")
        .unwrap()
        .lines()
        .map(|x| get_score(x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap()))
        .sum();

    println!("{}", total);
    Ok(())
}