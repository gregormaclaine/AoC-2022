enum Instruction {
    NOOP,
    ADD(i32),
}

fn main() -> std::io::Result<()> {
    let instructions: Vec<Instruction> = include_str!("../../../input/day-10.txt")
        .split('\n')
        .map(|x| {
            let mut parts = x.split(' ');
            match (parts.next(), parts.next()) {
                (Some("noop"), _) => Instruction::NOOP,
                (Some("addx"), Some(val)) => Instruction::ADD(val.parse().unwrap()),
                (_, _) => panic!("Invalid Instructions"),
            }
        })
        .collect();

    let mut register: i32 = 1;
    let mut cycle: i32 = 0;
    let mut total_score: i32 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::NOOP => {
                if cycle % 40 == 19 {
                    total_score += (cycle + 1) * register;
                }
            }
            Instruction::ADD(val) => {
                if cycle % 40 == 19 {
                    total_score += (cycle + 1) * register;
                } else if cycle % 40 == 18 {
                    total_score += (cycle + 2) * register;
                }
                cycle += 1;
                register += val;
            }
        }
        cycle += 1;
    }

    println!("{}", total_score);
    Ok(())
}
