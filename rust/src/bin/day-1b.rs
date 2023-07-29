fn main() -> std::io::Result<()> {
    let mut lines: Vec<u32> = include_str!("../../../input/day-1.txt")
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();

    lines.sort();

    let total: u32 = (&lines[(lines.len() - 3)..lines.len()]).into_iter().sum();

    println!("{}", total);
    Ok(())
}
