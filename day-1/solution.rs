use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let lines: Vec<String> = read_to_string("day-1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut running_total: i32 = 0;
    let mut elves: Vec<i32> = vec![];

    for line in lines {
        match line.as_str() {
            "" => {
                elves.push(running_total);
                running_total = 0;
            }
            _ => {
                running_total += line.parse::<i32>().unwrap();
            }
        }
    }

    let mut max_i: usize = 0;
    for i in 1..elves.len() {
        if elves[i] > elves[max_i] {
            max_i = i;
        }
    }

    println!("{}", elves[max_i]);
    Ok(())
}