use std::collections::HashMap;

const MAX_WIDTH: i32 = 1000;

fn main() -> std::io::Result<()> {
    let grid: Vec<(&str, u32)> = include_str!("../../../input/day-9.txt")
        .split('\n')
        .map(|line| {
            let parts: &mut (dyn Iterator<Item = &str>) = &mut line.split(' ');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut visited: HashMap<i32, bool> = HashMap::from([(0, true)]);
    let mut count: u32 = 1;

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    for movement in grid.into_iter() {
        for _ in 0..movement.1 {
            let new_head: (i32, i32) = match movement.0 {
                "R" => (head.0 + 1, head.1),
                "L" => (head.0 - 1, head.1),
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                _ => (0, 0),
            };

            let dist_sq: i32 = (new_head.0 - tail.0).pow(2) + (new_head.1 - tail.1).pow(2);
            if dist_sq >= 4 {
                tail = (head.0, head.1);
                let tail_val: i32 = tail.0 + tail.1 * MAX_WIDTH;
                if !visited.contains_key(&tail_val) {
                    visited.insert(tail_val, true);
                    count += 1;
                }
            }

            head = new_head;
        }
    }

    println!("{}", count);
    Ok(())
}
