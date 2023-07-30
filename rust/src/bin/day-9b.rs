use std::collections::HashMap;

const MAX_WIDTH: i32 = 1000;

fn dist(head: &(i32, i32), tail: &(i32, i32)) -> i32 {
    (head.0 - tail.0).pow(2) + (head.1 - tail.1).pow(2)
}

fn hash_val(pos: &(i32, i32)) -> i32 {
    pos.0 + pos.1 * MAX_WIDTH
}

fn constrain(val: i32) -> i32 {
    match val {
        0 => 0,
        ..=-1 => -1,
        1.. => 1,
    }
}

// fn draw_grid(head: &(i32, i32), knots: &Vec<(i32, i32)>, size: i32) {
//     println!("");
//     let w = size >> 1;
//     for j in (-w..w).rev() {
//         for i in -w..w {
//             let val = if head.0 == i && head.1 == j {
//                 "H".to_owned()
//             } else {
//                 let mut num: usize = 0;
//                 for k_i in 0..knots.len() {
//                     if knots[k_i].0 == i && knots[k_i].1 == j {
//                         num = k_i + 1;
//                         break;
//                     }
//                 }
//                 if num > 0 {
//                     num.to_string()
//                 } else if i == 0 && j == 0 {
//                     "s".to_owned()
//                 } else {
//                     ".".to_owned()
//                 }
//             };
//             print!("{} ", val);
//         }
//         println!("");
//     }
// }

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

    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 9];

    for movement in grid.into_iter() {
        for _ in 0..movement.1 {
            let new_head: (i32, i32) = match movement.0 {
                "R" => (head.0 + 1, head.1),
                "L" => (head.0 - 1, head.1),
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                _ => (0, 0),
            };

            //    M  A
            //    M  A->B
            //    M  _  B
            //    M->A  B
            //    _  M  B
            // C->_  M  B

            for i in 0..knots.len() {
                let new_head_knot = if i == 0 { new_head } else { knots[i - 1] };
                // println!(
                //     "{} -> {:?} {:?}",
                //     i, knots[i], new_head_knot
                // );
                match dist(&knots[i], &new_head_knot) {
                    4 | 5 | 8 => {
                        // Moved far enough to not be connected
                        let dir: (i32, i32) =
                            (new_head_knot.0 - knots[i].0, new_head_knot.1 - knots[i].1);
                        let new_knot_pos: (i32, i32) =
                            (knots[i].0 + constrain(dir.0), knots[i].1 + constrain(dir.1));
                        knots[i] = new_knot_pos;
                    }
                    0 | 1 | 2 => {
                        // Is still close enough to knot: therefore no change
                    }
                    _ => {} // Not possible
                }
            }

            let last_knot_key: i32 = hash_val(knots.last().unwrap());
            if !visited.contains_key(&last_knot_key) {
                visited.insert(last_knot_key, true);
                count += 1;
            }

            head = new_head;
            // println!("{:?}", movement);
            // draw_grid(&head, &knots, 25);
        }
        // println!("{:?}, {}", knots, count);
        // draw_grid(&head, &knots, 25);
    }

    println!("{}", count);
    Ok(())
}
