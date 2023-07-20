use std::num::ParseIntError;

fn parse_line(line: &str) -> Result<bool, ParseIntError> {
    let split: Vec<Vec<u32>> = line.split(',')
        .map(|x| x.split('-').map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    if split[0][0] == split[1][0] {
        Ok(true)
    } else if split[0][0] < split[1][0] {
        Ok(split[0][1] >= split[1][1])
    } else {
        Ok(split[0][1] <= split[1][1])
    }
}

fn main() -> std::io::Result<()> {
    let count: u32 = include_str!("../../../input/day-4.txt")
        .split("\n")
        .map(|x| if parse_line(x).unwrap() {1} else {0})
        .sum();
    
    println!("{}", count);
    Ok(())
}