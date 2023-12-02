pub mod util;

use std::cmp::max;

fn game_power(game: &str) -> i32 {
    let mut r_min = 0;
    let mut g_min = 0;
    let mut b_min = 0;

    for draw in game.split("; ") {
        for set in draw.split(", ") {
            let mut n_color = set.split(" ");
            let n = n_color
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let color = n_color.next().unwrap();
            match color {
                "red" => r_min = max(r_min, n),
                "green" => g_min = max(g_min, n),
                "blue" => b_min = max(b_min, n),
                _ => ()
            };
        }
    }
    
    r_min * g_min * b_min
}

fn main() {
    let input = util::read_input();
    let games = util::split_lines_str(input);
    
    let mut sum = 0;

    for game in games {
        let split_pos = match game.find(":") {
            Some(v) => v,
            None => { println!("What?"); 0 },
        };
        
        let power = game_power(&game[split_pos+2..]);
        sum += power;
    }

    util::write_output(sum);
}
