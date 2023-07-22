use std::num::ParseIntError;

type CMDLine<'a> = (&'a str, &'a str, Option<&'a str>);

fn calc_folder_size(
    lines: &Vec<CMDLine>,
    cur_line: &mut usize,
    sizes: &mut Vec<u32>) -> Result<u32, ParseIntError> {

    let mut size: u32 = 0;

    while *cur_line < lines.len() {
        // println!("{:?}", lines[*cur_line]);
        
        match lines[*cur_line] {
            ("$", "cd", Some("..")) => break,
            ("$", "cd", Some(_)) => {
                *cur_line += 1;
                size += calc_folder_size(lines, cur_line, sizes).unwrap();
            },
            ("$", "ls", _) => {},
            ("dir", _, _) => {},
            (s, _, _) => {
                size += s.parse::<u32>().unwrap();
            }
        }

        *cur_line += 1;
    }

    sizes.push(size);
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

    let mut cur_line: usize = 1;
    let mut sizes: Vec<u32> = Vec::new();

    let total = calc_folder_size(&lines, &mut cur_line, &mut sizes).unwrap();

    sizes.sort();
    let needed_space = total - 40_000_000;
    for s in sizes {
        if s >= needed_space {
            println!("{:?}", s);
            break;
        }
    }

    Ok(())
}