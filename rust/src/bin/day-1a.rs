fn main() -> std::io::Result<()> {
    let total: u32 = include_str!("../../../input/day-1.txt")
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.parse::<u32>().unwrap()).sum())
        .max().unwrap();
    
    println!("{}", total);
    Ok(())
}