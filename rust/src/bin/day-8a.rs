type BinLength = u128;

fn get_visible_indexes(arr: &Vec<u32>) -> BinLength {
    let len = arr.len();

    let mut vis_from_left: BinLength = 1 << (len - 1);
    let mut vis_from_right: BinLength = 1;

    let mut max = arr[0];

    for i in 1..len {
        if max < arr[i] {
            max = arr[i];
            vis_from_left += 1 << (len - i - 1);
        }
    }

    max = arr[len - 1];

    for i in (0..(len - 1)).rev() {
        if max < arr[i] {
            max = arr[i];
            vis_from_right += 1 << (len - i - 1);
        }
    }

    return vis_from_right | vis_from_left;
}

fn main() -> std::io::Result<()> {
    let grid: Vec<Vec<u32>> = include_str!("../../../input/day-8.txt")
        .split('\n')
        .map(|line| line.chars().map(|x| x as u32 - 0x30).collect())
        .collect();
    // println!("{:?}", grid);

    let row_visibles: Vec<BinLength> = grid
        .iter()
        // .skip(1).take(grid.len() - 2)
        .map(get_visible_indexes)
        .collect();

    let column_visibles: Vec<BinLength> = (0..grid.len())
        .map(|i| {
            let column: Vec<u32> = grid.iter().map(|line| line[i]).collect();
            get_visible_indexes(&column)
        })
        .collect();

    let transposed_columns: Vec<BinLength> = (0..column_visibles.len())
        .map(|i| {
            column_visibles
                .iter()
                .enumerate()
                .map(|(j, col)| {
                    (col >> (column_visibles.len() - 1 - i) & 1) << (column_visibles.len() - 1 - j)
                })
                .sum()
        })
        .collect();

    let visible: u32 = row_visibles
        .into_iter()
        .zip(transposed_columns)
        .map(|(row, col_t)| (row | col_t).count_ones())
        .sum();

    println!("{}", visible);
    Ok(())
}
