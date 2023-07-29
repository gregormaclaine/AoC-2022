fn scale_scores_from_left(heights: &Vec<&u32>, scalars: &mut Vec<&mut u32>) {
    *scalars[0] = 0;

    for i in 2..heights.len() {
        for j in (0..i).rev() {
            if j == 0 {
                *scalars[i] *= i as u32;
            } else if heights[j] >= heights[i] {
                *scalars[i] *= (i - j) as u32;
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let grid: Vec<u32> = include_str!("../../../input/day-8.txt")
        .split('\n')
        .flat_map(|line| line.chars().map(|x| x as u32 - 0x30).collect::<Vec<u32>>())
        .collect();

    let width: usize = f32::sqrt(grid.len() as f32) as usize;

    // Flattened 2d array of same size as grid of 1s
    let mut scalars: Vec<u32> = vec![1; grid.len()];

    for i in 0..width {
        let left_row: Vec<&u32> = grid.iter().skip(i * width).take(width).collect();
        let mut left_row_scalars: Vec<&mut u32> =
            scalars.iter_mut().skip(i * width).take(width).collect();
        scale_scores_from_left(&left_row, &mut left_row_scalars);

        let right_row: Vec<&u32> = grid.iter().skip(i * width).take(width).rev().collect();
        let mut right_row_scalars: Vec<&mut u32> = scalars
            .iter_mut()
            .skip(i * width)
            .take(width)
            .rev()
            .collect();
        scale_scores_from_left(&right_row, &mut right_row_scalars);

        let up_column: Vec<&u32> = grid.iter().skip(i).step_by(width).collect();
        let mut up_column_scalars: Vec<&mut u32> =
            scalars.iter_mut().skip(i).step_by(width).collect();
        scale_scores_from_left(&up_column, &mut up_column_scalars);

        let down_column: Vec<&u32> = grid.iter().skip(i).step_by(width).rev().collect();
        let mut down_column_scalars: Vec<&mut u32> =
            scalars.iter_mut().skip(i).step_by(width).rev().collect();
        scale_scores_from_left(&down_column, &mut down_column_scalars);
    }

    println!("{}", scalars.into_iter().max().unwrap());
    Ok(())
}
