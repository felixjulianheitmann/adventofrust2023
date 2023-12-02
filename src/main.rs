pub mod util;

fn is_game_impossible(game: &str, limits: &[i32]) -> bool {

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
                "red" => if n > limits[0] { return true } else { () } ,
                "green" => if n > limits[1] { return true } else { () },
                "blue" => if n > limits[2] { return true } else { () },
                _ => ()
            };
        }
    }
    
    false
}

static LIMITS: &[i32] = &[12, 13, 14];

fn main() {
    let input = util::read_input();
    let games = util::split_lines_str(input);
    
    let mut sum = 0;

    for game in games {
        let split_pos = match game.find(":") {
            Some(v) => v,
            None => { println!("What?"); 0 },
        };
        let game_id = game[5..split_pos]
            .parse::<i32>()
            .unwrap();
        if !is_game_impossible(&game[split_pos+2..], LIMITS) {
            sum += game_id;
            println!("game id: {}", game_id);
        }
    }

    util::write_output(sum);
}
