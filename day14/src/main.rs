use regex::Regex;
use std::collections::HashMap;

struct Robot {
    px: u8,
    py: u8,
    vx: i8,
    vy: i8,
}

fn main() {
    let input = read_input();

    let width: u8 = 101;
    let height: u8 = 103;

    let positions: Vec<(u8, u8)> = calculate_positions(&input, 100, width, height);

    let quadrants: HashMap<u8, usize> = positions
        .iter()
        .map(|&position| to_quadrant(position, width, height))
        .filter(|quadrant| quadrant.is_some())
        .map(|quadrant| quadrant.unwrap())
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        });

    let part1 = quadrants.values().fold(1, |acc, &val| acc * val);

    println!("{}", part1);

    for seconds in 0..10000 {
        let positions = calculate_positions(&input, seconds, width, height);

        if detect_tree(&positions) {
            println!("At {} seconds:", seconds);
            create_image(&positions, width, height);
        }
    }
}

fn detect_tree(positions: &Vec<(u8, u8)>) -> bool {
    positions.iter().any(|(x, y)| {
        for i in 1..=10 {
            if !positions.contains(&(x + i, *y)) {
                return false;
            }
        }

        true
    })
}

fn create_image(positions: &Vec<(u8, u8)>, width: u8, height: u8) {
    let mut image = String::new();

    image.push_str("\n");
    image.push_str(&*(0..width).map(|_| "=").collect::<String>());
    image.push_str("\n");

    for line in 0..height {
        for col in 0..width {
            if positions.contains(&(col, line)) {
                image.push_str("#");
            } else {
                image.push_str(" ");
            }
        }
        image.push_str("\n");
    }

    image.push_str(&*(0..width).map(|_| "=").collect::<String>());
    image.push_str("\n");

    print!("{}", image);
}

fn calculate_positions(
    robots: &Vec<Robot>,
    seconds: usize,
    width: u8,
    height: u8,
) -> Vec<(u8, u8)> {
    robots
        .iter()
        .map(|robot| {
            (
                robot.px as i64 + seconds as i64 * robot.vx as i64,
                robot.py as i64 + seconds as i64 * robot.vy as i64,
            )
        })
        .map(|(px, py)| (px % width as i64, py % height as i64))
        .map(|(px, py)| {
            (
                if px >= 0 {
                    px as u8
                } else {
                    (px + width as i64) as u8
                },
                if py >= 0 {
                    py as u8
                } else {
                    (py + height as i64) as u8
                },
            )
        })
        .collect()
}

fn to_quadrant(position: (u8, u8), width: u8, height: u8) -> Option<u8> {
    // q1: (0,0) -> (width/2, height/2)
    // q2: (width/2,0) -> (width, height/2)
    // q3: (0,height/2) -> (width/2, height)
    // q4: (width/2,height/2) -> (width, height)

    if position.0 < width / 2 && position.1 < height / 2 {
        return Some(1);
    } else if position.0 >= width / 2 + 1 && position.1 < height / 2 {
        return Some(2);
    } else if position.0 < width / 2 && position.1 >= height / 2 + 1 {
        return Some(3);
    } else if position.0 >= (width / 2 + 1) && position.1 >= (height / 2 + 1) {
        return Some(4);
    }

    None
}

fn read_input() -> Vec<Robot> {
    let line_regex = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();

    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| {
            let captures = line_regex.captures(line).expect("Invalid line");

            Robot {
                px: captures
                    .get(1)
                    .expect("px")
                    .as_str()
                    .parse()
                    .expect("parse number"),
                py: captures
                    .get(2)
                    .expect("py")
                    .as_str()
                    .parse()
                    .expect("parse number"),
                vx: captures
                    .get(3)
                    .expect("vx")
                    .as_str()
                    .parse()
                    .expect("parse number"),
                vy: captures
                    .get(4)
                    .expect("vy")
                    .as_str()
                    .parse()
                    .expect("parse number"),
            }
        })
        .collect()
}
