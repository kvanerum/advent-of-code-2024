use regex::Regex;

struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn main() {
    let input = read_input();

    let mut result: u64 = input
        .iter()
        .map(|game| play(game, false))
        .filter_map(|game| game)
        .sum();

    println!("Part 1: {}", result);

    result = input
        .iter()
        .map(|game| play(game, true))
        .filter_map(|game| game)
        .sum();

    println!("Part 2: {}", result);
}

fn play(game: &Game, part_2: bool) -> Option<u64> {
    let mut prize_x = game.prize.0;
    let mut prize_y = game.prize.1;

    if part_2 {
        prize_x += 10000000000000;
        prize_y += 10000000000000;
    }

    let button_b = (game.button_a.1 as f64 * prize_x as f64 - (game.button_a.0 * prize_y) as f64)
        / ((game.button_a.1 * game.button_b.0) as f64 - (game.button_a.0 * game.button_b.1) as f64);

    let button_a = (prize_x as f64 - button_b * game.button_b.0 as f64) / game.button_a.0 as f64;

    if button_a.fract() != 0.0 || button_b.fract() != 0.0 {
        return None;
    }

    Some(button_a as u64 * 3 + button_b as u64)
}

fn parse_line(line: &str, regex: &Regex) -> (u64, u64) {
    let captures = regex.captures(line).expect("Invalid line");

    let x = captures
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u64>()
        .expect("Number");
    let y = captures
        .get(2)
        .unwrap()
        .as_str()
        .parse::<u64>()
        .expect("Number");

    (x, y)
}

fn read_input() -> Vec<Game> {
    let button_regex = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .map(|game| {
            let mut lines = game.lines();

            Game {
                button_a: parse_line(lines.next().unwrap(), &button_regex),
                button_b: parse_line(lines.next().unwrap(), &button_regex),
                prize: parse_line(lines.next().unwrap(), &prize_regex),
            }
        })
        .collect()
}
