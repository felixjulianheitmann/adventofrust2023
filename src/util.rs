pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Something went wrong reading the input...")
}

pub fn split_lines<T>(text: String) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    text.split('\n').map(|l| l.parse().unwrap()).collect()
}

pub fn split_lines_str(text: String) -> Vec<String> {
    text.split('\n').map(|l| String::from(l)).collect()
}

pub fn split_at<'a, T>(token: &str, text: String) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    text.split(token).map(|l| l.parse().unwrap()).collect()
}

pub fn write_output<T>(out: T)
where
    T: std::fmt::Display,
{
    let msg = format!("The result is: {}", out.to_string());
    println!("{}",msg);
    std::fs::write("output.txt", msg)
        .expect("Writing to output.txt failed...")
}

fn neighbor_checker<'a, F>(grid: &'a Vec<String>, checker: &'a F, default_out_of_bounds: bool) -> impl Fn(i32, i32) -> bool + 'a
where
    F: Fn(i32, i32, char, &str) -> bool,
{
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    
    return move |x: i32, y: i32| -> bool {
        if x >= 0 && x < w && y >= 0 && y < h {
            let line = &grid[y as usize];
            let c = line.chars().nth(x as usize).unwrap();
            return checker(x, y, c, &line.as_str())
        }
        default_out_of_bounds
    }

}

pub fn check_any_neighbors<F>(x: usize, y: usize, grid: &Vec<String>, checker: F) -> bool
where
    F: Fn(i32, i32, char, &str) -> bool
{
    let check = neighbor_checker(grid, &checker, false);

    if check((x - 1) as i32, (y - 1) as i32)
    || check((x + 1) as i32, (y - 1) as i32)
    || check(x as i32, (y - 1) as i32)
    || check((x - 1) as i32, (y + 1) as i32)
    || check((x + 1) as i32, (y + 1) as i32)
    || check(x as i32, (y + 1) as i32)
    || check((x - 1) as i32, y as i32)
    || check((x + 1) as i32, y as i32) {
        return true
    }
    false

}

pub fn check_all_neighbors<F>(x: usize, y: usize, grid: &Vec<String>, checker: F) -> bool
where
    F: Fn(i32, i32, char, &str) -> bool
{
    let check = neighbor_checker(grid, &checker, false);

    if check((x - 1) as i32, (y - 1) as i32)
    && check((x + 1) as i32, (y - 1) as i32)
    && check(x as i32, (y - 1) as i32)
    && check((x - 1) as i32, (y + 1) as i32)
    && check((x + 1) as i32, (y + 1) as i32)
    && check(x as i32, (y + 1) as i32)
    && check((x - 1) as i32, y as i32)
    && check((x + 1) as i32, y as i32) {
        return true
    }
    false
}

pub fn for_each_neighbor<F>(x: usize, y: usize, grid: &Vec<String>, f: F)
where
    F: Fn(i32, i32, char, &str) -> ()
{

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let runner = |x: i32, y: i32| {
        if x >= 0 && x < w && y >= 0 && y < h {
            let line = &grid[y as usize];
            let c = line.chars().nth(x as usize).unwrap();
            return f(x, y, c, &line.as_str())
        }
    };

    runner((x - 1) as i32, (y - 1) as i32);
    runner((x + 1) as i32, (y - 1) as i32);
    runner(x as i32, (y - 1) as i32);
    runner((x - 1) as i32, (y + 1) as i32);
    runner((x + 1) as i32, (y + 1) as i32);
    runner(x as i32, (y + 1) as i32);
    runner((x - 1) as i32, y as i32);
    runner((x + 1) as i32, y as i32);

}