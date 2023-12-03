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
                Err(_) => {println!("couldn't parse: {}", subline); continue},
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
    // check for sorrounding symbols
    'number_loop: for number in numbers {
        for (x, y) in number.coords {
            let x = x as i32;
            let y = y as i32;

            let check = |sum: &mut i32, x: i32, y: i32, h, w| -> bool {
                if x >= 0 && x < w && y >= 0 && y < h {
                    let line = &lines[y as usize];
                    let c = line.chars().nth(x as usize).unwrap();
                    if !c.is_numeric() && c != '.' {
                        *sum += number.value;
                        return true;
                    }
                }
                false
            };

            // check all neighbors for symbols
            if check(&mut sum, x - 1, y - 1, h, w)
                || check(&mut sum, x + 1, y - 1, h, w)
                || check(&mut sum, x, y - 1, h, w)
                || check(&mut sum, x - 1, y + 1, h, w)
                || check(&mut sum, x + 1, y + 1, h, w)
                || check(&mut sum, x, y + 1, h, w)
                || check(&mut sum, x - 1, y, h, w)
                || check(&mut sum, x + 1, y, h, w)
            {
                // println!("Number {} has an adjacent symbol", number.value);
                continue 'number_loop;
            }
        }
    }

    util::write_output(sum);
}
