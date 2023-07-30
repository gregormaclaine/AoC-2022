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
    let mut running_addx: bool = false;
    let mut instruction_i: usize = 0;

    for i in 0..240 {
        let is_lit: bool = register.abs_diff(i % 40) < 2;
        print!("{}", if is_lit { "#" } else { "." });
        if i % 40 == 39 {
            println!("")
        }

        if running_addx {
            running_addx = false;
            match instructions[instruction_i - 1] {
                Instruction::ADD(val) => register += val,
                Instruction::NOOP => panic!("Something went wrong"),
            }
            continue;
        }

        match instructions[instruction_i] {
            Instruction::NOOP => {}
            Instruction::ADD(_) => {
                running_addx = true;
            }
        }
        instruction_i += 1;
    }

    println!("");
    Ok(())
}
