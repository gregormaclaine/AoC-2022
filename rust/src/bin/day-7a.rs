use std::num::ParseIntError;

type CMDLine<'a> = (&'a str, &'a str, Option<&'a str>);

fn calc_folder_size(
    lines: &Vec<CMDLine>,
    cur_line: &mut usize,
    total: &mut u32) -> Result<u32, ParseIntError> {

    let mut size: u32 = 0;

    while *cur_line < lines.len() {
        // println!("{:?}", lines[*cur_line]);
        
        match lines[*cur_line] {
            ("$", "cd", Some("..")) => break,
            ("$", "cd", Some(_)) => {
                *cur_line += 1;
                size += calc_folder_size(lines, cur_line, total).unwrap();
            },
            ("$", "ls", _) => {},
            ("dir", _, _) => {},
            (s, _, _) => {
                size += s.parse::<u32>().unwrap();
            }
        }

        *cur_line += 1;
    }

    if size < 100_000 { *total += size; }
    Ok(size)
}

fn main() -> std::io::Result<()> {
    let lines: Vec<CMDLine> = include_str!("../../../input/day-7.txt")
        .split('\n')
        .map(|x| {
            let mut parts = x.split(' ');
            (parts.next().unwrap(), parts.next().unwrap(), parts.next())
        })
        .collect();

    let mut total: u32 = 0;
    let mut cur_line: usize = 1;

    let _ = calc_folder_size(&lines, &mut cur_line, &mut total);

    println!("{}", total);
    Ok(())
}