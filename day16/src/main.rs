use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let (map, start_position, end_position) = read_input();

    let mut lowest_score_cache: HashMap<((usize, usize), &Direction), usize> = HashMap::new();
    let turns_map: HashMap<Direction, Vec<Direction>> = [
        (Direction::North, vec![Direction::East, Direction::West]),
        (Direction::East, vec![Direction::South, Direction::North]),
        (Direction::South, vec![Direction::West, Direction::East]),
        (Direction::West, vec![Direction::North, Direction::South]),
    ]
    .into_iter()
    .collect();

    let mut minimum_score = usize::MAX;
    step(
        start_position,
        &Direction::East,
        0,
        &map,
        &mut lowest_score_cache,
        &turns_map,
        end_position,
        &mut minimum_score,
    );

    let result = end_position_min_score(&lowest_score_cache, end_position);

    println!("{}", result);
}

fn end_position_min_score(
    lowest_score_cache: &HashMap<((usize, usize), &Direction), usize>,
    end_position: (usize, usize),
) -> usize {
    lowest_score_cache
        .iter()
        .filter(|((position, _), _)| *position == end_position)
        .map(|(_, &score)| score)
        .min()
        .unwrap_or(usize::MAX)
}

fn step<'a>(
    position: (usize, usize),
    direction: &'a Direction,
    current_score: usize,
    map: &Vec<Vec<Tile>>,
    lowest_score_cache: &mut HashMap<((usize, usize), &'a Direction), usize>,
    turns_map: &'a HashMap<Direction, Vec<Direction>>,
    end_position: (usize, usize),
    minimum_score: &mut usize,
) {
    if current_score > *minimum_score {
        return;
    }

    let old_score = lowest_score_cache
        .get(&(position, direction))
        .unwrap_or(&usize::MAX);

    if old_score < &current_score {
        return;
    }

    if position == end_position {
        println!(
            "looks like we made it to the end with score {}",
            current_score
        );
        *minimum_score = current_score;
    }

    lowest_score_cache.insert((position, direction), current_score);

    // try step
    let next_position = next_position(position, direction);
    if map[next_position.0][next_position.1] == Tile::Empty {
        step(
            next_position,
            direction,
            current_score + 1,
            map,
            lowest_score_cache,
            turns_map,
            end_position,
            minimum_score,
        );
    }

    // try turns
    turns_map
        .get(&direction)
        .expect("next turns")
        .iter()
        .for_each(|next_direction| {
            step(
                position,
                next_direction,
                current_score + 1000,
                map,
                lowest_score_cache,
                turns_map,
                end_position,
                minimum_score,
            );
        })
}

fn next_position(position: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (position.0 - 1, position.1),
        Direction::South => (position.0 + 1, position.1),
        Direction::East => (position.0, position.1 + 1),
        Direction::West => (position.0, position.1 - 1),
    }
}

fn read_input() -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let parts = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .collect::<Vec<_>>();

    let tile_map: HashMap<char, Tile> = [('#', Tile::Wall), ('.', Tile::Empty)]
        .into_iter()
        .collect();

    let mut start_position: Option<(usize, usize)> = None;
    let mut end_position: Option<(usize, usize)> = None;
    let map: Vec<Vec<Tile>> = parts[0]
        .split("\n")
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, c)| {
                    if c == 'S' {
                        start_position = Some((row, column));
                        Tile::Empty
                    } else if c == 'E' {
                        end_position = Some((row, column));
                        Tile::Empty
                    } else {
                        tile_map.get(&c).expect("tile").clone()
                    }
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    (
        map,
        start_position.expect("starting position"),
        end_position.expect("ending position"),
    )
}
