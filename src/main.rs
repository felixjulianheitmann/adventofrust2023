pub mod util;

const NUMBERS:  [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn first_digit(line: &str, reverse: bool) -> i32 {
    for pos in 0..line.len() {
        let mut idx = pos;
        if reverse {
            idx = line.len() - pos - 1;
        }

        let tmp = &line[idx..];

        // Check for number in string
        let ch = match tmp.chars().next() {
            Some(c) => c,
            None => return -1,
        };
        if ch.is_numeric() {
            return ch
                .to_string()
                .parse::<i32>()
                .expect(format!("failed to parse digit: {}", ch)
                .as_str())
        }

        // check for one of the string representations
        for (n, n_str) in NUMBERS.into_iter().enumerate() {
            if tmp.starts_with(n_str) {
                return n as i32 + 1
            }

        }

    }

    -1
}

fn main() {
    let input = util::read_input();
    let input = util::split_lines::<String>(input);

    let mut sum = 0;
    for line in input.iter() {
        let first = first_digit(line, false);
        let last = first_digit(line, true);

        sum += first * 10 + last;
        println!("Sum: {}\t+{}\t= {}", first * 10, last, sum)
    }

    util::write_output(sum);
}
