use itertools::iproduct;
use std::collections::HashSet;

fn main() {
    let input = read_input();

    let trail_head: u8 = 0;

    let result: (usize, usize) = iproduct!(0..input.len(), 0..input[0].len())
        .into_iter()
        .filter(|&(row, column)| input[row][column] == trail_head)
        .map(|position| {
            let mut aggregator: Vec<(usize, usize)> = Vec::new();

            calculate_score(position, &input, &mut aggregator);

            (
                aggregator.iter().collect::<HashSet<_>>().len(),
                aggregator.len(),
            )
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("{}", result.0);
    println!("{}", result.1);
}

fn calculate_score(
    current_position: (usize, usize),
    map: &Vec<Vec<u8>>,
    aggregator: &mut Vec<(usize, usize)>,
) {
    let height = map[current_position.0][current_position.1];

    if height == 9 {
        aggregator.push((current_position.0, current_position.1));
        return;
    }

    // up
    if current_position.0 > 0 && map[current_position.0 - 1][current_position.1] == height + 1 {
        calculate_score(
            (current_position.0 - 1, current_position.1),
            map,
            aggregator,
        );
    }

    // down
    if current_position.0 < map.len() - 1
        && map[current_position.0 + 1][current_position.1] == height + 1
    {
        calculate_score(
            (current_position.0 + 1, current_position.1),
            map,
            aggregator,
        );
    }

    // left
    if current_position.1 > 0 && map[current_position.0][current_position.1 - 1] == height + 1 {
        calculate_score(
            (current_position.0, current_position.1 - 1),
            map,
            aggregator,
        );
    }

    // right
    if current_position.1 < map[0].len() - 1
        && map[current_position.0][current_position.1 + 1] == height + 1
    {
        calculate_score(
            (current_position.0, current_position.1 + 1),
            map,
            aggregator,
        );
    }
}

fn read_input() -> Vec<Vec<u8>> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| line.chars().map(|x| x as u8 - b'0').collect())
        .collect()
}
