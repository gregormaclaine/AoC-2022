use std::num::ParseIntError;

fn parse_move_line(line: &str) -> Result<(u32, u32, u32), ParseIntError> {
    let words: Vec<&str> = line.split(' ').collect();
    Ok((
        words[1].parse::<u32>().unwrap(),
        words[3].parse::<u32>().unwrap(),
        words[5].parse::<u32>().unwrap(),
    ))
}

fn main() -> std::io::Result<()> {
    let sections: Vec<Vec<&str>> = include_str!("../../../input/day-5.txt")
        .split("\n\n")
        .map(|x| x.split('\n').collect())
        .collect();

    // Initialise starting position
    let init_lines = sections[0].to_owned();
    let num_piles = (init_lines[0].len() + 1) >> 2;
    let mut piles: Vec<Vec<char>> = vec![Vec::new(); num_piles];

    for line in init_lines.into_iter().rev() {
        for (i, c) in line.chars().enumerate() {
            if (i + 3) % 4 == 0 {
                match c {
                    'A'..='Z' => piles[(i - 1) >> 2].push(c),
                    _ => continue,
                }
            }
        }
    }
    // println!("{:?}", piles);

    // Enact moves
    sections[1]
        .to_owned()
        .into_iter()
        .map(|x| parse_move_line(x).unwrap())
        .for_each(|(count, from, to)| {
            let mut stack: Vec<char> = Vec::with_capacity(count as usize);

            for _ in 0..count {
                stack.push(piles[(from - 1) as usize].pop().unwrap());
            }

            for _ in 0..count {
                piles[(to - 1) as usize].push(stack.pop().unwrap());
            }
        });

    let top_items: String = piles.iter().map(|x| x.last().unwrap()).collect();
    println!("{:?}", top_items);
    Ok(())
}
