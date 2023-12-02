pub mod util;

fn main() {
    let input = util::read_input();
    let input = util::split_lines::<String>(input);

    let mut sum = 0;
    for line in input.iter() {
        let mut first_digit = '\n';
        let mut last_digit = '\n';
        for char in line.chars() {
            if char.is_numeric() {
                first_digit = char;
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                last_digit = char;
                break;
            }
        }

        let coordinate = String::from_iter([first_digit, last_digit].iter());
        sum += match coordinate.parse::<i32>() {
            Ok(s) => s,
            Err(e) => panic!("failed to parse coordinates: {} - {}", coordinate, e),
        };
    }

    util::write_output(sum);
}
