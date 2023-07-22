fn main() -> std::io::Result<()> {
    let input: Vec<char> = include_str!("../../../input/day-6.txt").chars().collect();
    let mut bin_map: u32 = 0x0000;

    for i in 0..14 {
        bin_map ^= 1 << (input[i] as u32 - 'a' as u32);
    }

    for i in 14..input.len() {
        if bin_map.count_ones() == 14 {
            println!("{}", i);
            break;
        }

        bin_map ^= 1 << (input[i] as u32 - 'a' as u32) ^
                   1 << (input[i - 14] as u32 - 'a' as u32);
    }

    Ok(())
}