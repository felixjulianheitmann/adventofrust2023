pub mod util;

struct Number {
    value: i32,
    coords: Vec<(usize, usize)>,
}

fn main() {
    let input = util::read_input();
    let lines = util::split_lines_str(input);

    let mut numbers = vec![];

    // Find all number coordinates
    for (idx_y, line) in lines.iter().enumerate() {
        for idx_x in 0..line.len() {
            // skip the remaining digits of a number
            if idx_x > 0 && line.chars().nth(idx_x - 1).unwrap().is_numeric() {
                continue;
            }

            // find end of number
            let dot_pos = match line[idx_x..].chars().position(|c| !c.is_numeric()) {
                Some(x) => {
                    if x == 0 {
                        continue;
                    } else {
                        x
                    }
                }
                None => line.len() - idx_x,
            };

            // parse number
            let subline = &line[idx_x..idx_x + dot_pos];
            let num = match subline.parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    println!("couldn't parse: {}", subline);
                    continue;
                }
            };

            // collect number + coordinates
            let mut number = Number {
                value: num,
                coords: vec![],
            };
            for digit in idx_x..idx_x + dot_pos {
                number.coords.push((digit, idx_y));
            }
            numbers.push(number);
        }
    }

    let mut sum = 0;
    let h = lines.len() as i32;
    let w = lines[0].len() as i32;

    // Check all stars for numeric neihbors
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            let check = |h, w, x: i32, y: i32| -> bool {
                if x >= 0 && x < w && y >= 0 && y < h {
                    let line = &lines[y as usize];
                    let c = line.chars().nth(x as usize).unwrap();
                    if c.is_numeric() {
                        return true;
                    }
                }
                false
            };
            if c == '*' {
                let ul = check(h, w, x - 1, y - 1);
                let ur = check(h, w, x + 1, y - 1);
                let uc = check(h, w, x, y - 1);
                let bl = check(h, w, x - 1, y + 1);
                let br = check(h, w, x + 1, y + 1);
                let bc = check(h, w, x, y + 1);
                let cl = check(h, w, x - 1, y);
                let cb = check(h, w, x + 1, y);
                let mut n_numeric_neighbors = ul as i32
                    + ur as i32
                    + uc as i32
                    + bl as i32
                    + br as i32
                    + bc as i32
                    + cl as i32
                    + cb as i32;
                if bl && bc{
                    n_numeric_neighbors -= 1
                }
                if bc && br{
                    n_numeric_neighbors -= 1
                }
                if ul && uc{
                    n_numeric_neighbors -= 1
                }
                if uc && ur{
                    n_numeric_neighbors -= 1
                }
                if n_numeric_neighbors == 2 {
                    let mut neighbors = numbers.iter().filter(|n| {
                        n.coords.iter().any(|coord| {
                            coord == &((x - 1) as usize, (y - 1) as usize)
                                || coord == &((x + 1) as usize, (y - 1) as usize)
                                || coord == &(x as usize, (y - 1) as usize)
                                || coord == &((x - 1) as usize, (y + 1) as usize)
                                || coord == &((x + 1) as usize, (y + 1) as usize)
                                || coord == &(x as usize, (y + 1) as usize)
                                || coord == &((x - 1) as usize, y as usize)
                                || coord == &((x + 1) as usize, y as usize)
                        })
                    });

                    let gear_ratio =
                        neighbors.next().unwrap().value * neighbors.next().unwrap().value;
                    sum += gear_ratio;
                }
            }
        }
    }

    util::write_output(sum);
}
