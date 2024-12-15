use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Tile {
    Wall,
    Empty,
    Box,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let (mut map, moves, start_position) = read_input();

    let move_delta_map: HashMap<Move, (i8, i8)> = [
        (Move::Up, (-1, 0)),
        (Move::Down, (1, 0)),
        (Move::Right, (0, 1)),
        (Move::Left, (0, -1)),
    ]
    .into_iter()
    .collect();

    let mut current_position = start_position.clone();

    moves.iter().for_each(|m| {
        let delta = move_delta_map.get(&m).expect("move");
        let next_position = next_position(current_position, delta);
        let next_tile = &map[next_position.0][next_position.1];

        if next_tile == &Tile::Empty {
            current_position = next_position;
        } else if next_tile == &Tile::Box {
            if let Some(next_empty_position) = find_next_empty_position(&map, next_position, delta)
            {
                // move the box at next_position to next_empty_position
                map[next_position.0][next_position.1] = Tile::Empty;
                map[next_empty_position.0][next_empty_position.1] = Tile::Box;
                current_position = next_position;
            }
        }
    });

    println!("{}", calculate_all_boxes_gps_position(&map));
}

fn calculate_all_boxes_gps_position(map: &Vec<Vec<Tile>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(row, tiles)| {
            tiles
                .iter()
                .enumerate()
                .filter(|(_, tile)| **tile == Tile::Box)
                .map(|(column, _)| 100 * row + column)
                .sum::<usize>()
        })
        .sum()
}

fn find_next_empty_position(
    map: &Vec<Vec<Tile>>,
    position: (usize, usize),
    delta: &(i8, i8),
) -> Option<(usize, usize)> {
    let mut current_position = next_position(position, delta);

    while map[current_position.0][current_position.1] == Tile::Box {
        current_position = next_position(current_position, delta);
    }

    if map[current_position.0][current_position.1] == Tile::Empty {
        Some((current_position.0, current_position.1))
    } else {
        None
    }
}

fn next_position(position: (usize, usize), delta: &(i8, i8)) -> (usize, usize) {
    (
        (position.0 as i64 + delta.0 as i64) as usize,
        (position.1 as i64 + delta.1 as i64) as usize,
    )
}

fn read_input() -> (Vec<Vec<Tile>>, Vec<Move>, (usize, usize)) {
    let parts = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .collect::<Vec<_>>();

    let tile_map: HashMap<char, Tile> = [('#', Tile::Wall), ('.', Tile::Empty), ('O', Tile::Box)]
        .into_iter()
        .collect();

    let mut start_position: Option<(usize, usize)> = None;
    let map: Vec<Vec<Tile>> = parts[0]
        .split("\n")
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, c)| {
                    if c == '@' {
                        start_position = Some((row, column));
                        Tile::Empty
                    } else {
                        tile_map.get(&c).expect("tile}").clone()
                    }
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    let moves_map: HashMap<char, Move> = [
        ('^', Move::Up),
        ('v', Move::Down),
        ('>', Move::Right),
        ('<', Move::Left),
    ]
    .into_iter()
    .collect();

    let moves = parts[1]
        .replace("\n", "")
        .chars()
        .map(|c| moves_map.get(&c).expect("move").clone())
        .collect();

    (map, moves, start_position.expect("starting position"))
}
