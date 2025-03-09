use crate::Direction::{East, North, South, West};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let input = read_input();

    let regions = split_into_regions(&input);

    let mut result: usize = regions
        .iter()
        .map(|(_, positions)| {
            positions.len() * calculate_edges(positions, input[0].len(), input.len()).len()
        })
        .sum();

    println!("{}", result);

    result = regions
        .iter()
        .map(|(_, positions)| {
            positions.len() * count_sides(&calculate_edges(positions, input[0].len(), input.len()))
        })
        .sum();

    println!("{}", result);
}

fn calculate_edges(
    positions: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize, Direction)> {
    let mut result = HashSet::new();

    positions.iter().for_each(|position| {
        // up
        if position.0 as i32 - 1 < 0 || !positions.contains(&(position.0 - 1, position.1)) {
            result.insert((position.0, position.1, Direction::North));
        }
        // down
        if position.0 + 1 >= height || !positions.contains(&(position.0 + 1, position.1)) {
            result.insert((position.0, position.1, Direction::South));
        }
        // left
        if position.1 as i32 - 1 < 0 || !positions.contains(&(position.0, position.1 - 1)) {
            result.insert((position.0, position.1, Direction::West));
        }
        // right
        if position.1 + 1 >= width || !positions.contains(&(position.0, position.1 + 1)) {
            result.insert((position.0, position.1, Direction::East));
        }
    });

    result
}
fn count_sides(edges: &HashSet<(usize, usize, Direction)>) -> usize {
    count_horizontal_sides(
        &edges
            .iter()
            .filter(|(_, _, direction)| *direction == North)
            .map(|(row, col, _)| (row, col))
            .collect(),
    ) + count_horizontal_sides(
        &edges
            .iter()
            .filter(|(_, _, direction)| *direction == South)
            .map(|(row, col, _)| (row, col))
            .collect(),
    ) + count_vertical_sides(
        &edges
            .iter()
            .filter(|(_, _, direction)| *direction == East)
            .map(|(row, col, _)| (row, col))
            .collect(),
    ) + count_vertical_sides(
        &edges
            .iter()
            .filter(|(_, _, direction)| *direction == West)
            .map(|(row, col, _)| (row, col))
            .collect(),
    )
}

fn count_horizontal_sides(edges: &HashSet<(&usize, &usize)>) -> usize {
    // group by row
    let mut grouped: HashMap<usize, BTreeSet<usize>> = HashMap::new();

    edges.iter().for_each(|&(row, col)| {
        grouped
            .entry(*row)
            .or_insert_with(BTreeSet::new)
            .insert(*col);
    });

    let mut result = 0;

    grouped.values().for_each(|columns| {
        let mut previous = *columns.first().expect("first column");
        columns.iter().for_each(|&column| {
            if column != previous + 1 {
                result += 1;
            }

            previous = column;
        });
    });

    result
}

fn count_vertical_sides(edges: &HashSet<(&usize, &usize)>) -> usize {
    // group by column
    let mut grouped: HashMap<usize, BTreeSet<usize>> = HashMap::new();

    edges.iter().for_each(|&(row, col)| {
        grouped
            .entry(*col)
            .or_insert_with(BTreeSet::new)
            .insert(*row);
    });

    let mut result = 0;

    grouped.values().for_each(|rows| {
        let mut previous = *rows.first().expect("first row");
        rows.iter().for_each(|&column| {
            if column != previous + 1 {
                result += 1;
            }

            previous = column;
        });
    });

    result
}

fn split_into_regions(input: &Vec<Vec<char>>) -> Vec<(char, HashSet<(usize, usize)>)> {
    let mut regions = Vec::new();
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if !visited_positions.contains(&(row, col)) {
                let mut region_positions = HashSet::new();

                create_region((row, col), input, &mut region_positions);

                visited_positions.extend(&region_positions);
                regions.push((input[row][col], region_positions));
            }
        }
    }

    regions
}

fn create_region(
    current_position: (usize, usize),
    map: &Vec<Vec<char>>,
    region_positions: &mut HashSet<(usize, usize)>,
) {
    if region_positions.contains(&current_position) {
        return;
    }

    region_positions.insert(current_position);
    let current_char = map[current_position.0][current_position.1];

    // up
    if current_position.0 > 0 && map[current_position.0 - 1][current_position.1] == current_char {
        create_region(
            (current_position.0 - 1, current_position.1),
            map,
            region_positions,
        )
    }

    // down
    if current_position.0 < map.len() - 1
        && map[current_position.0 + 1][current_position.1] == current_char
    {
        create_region(
            (current_position.0 + 1, current_position.1),
            map,
            region_positions,
        );
    }

    // left
    if current_position.1 > 0 && map[current_position.0][current_position.1 - 1] == current_char {
        create_region(
            (current_position.0, current_position.1 - 1),
            map,
            region_positions,
        );
    }

    // right
    if current_position.1 < map[0].len() - 1
        && map[current_position.0][current_position.1 + 1] == current_char
    {
        create_region(
            (current_position.0, current_position.1 + 1),
            map,
            region_positions,
        );
    }
}

fn read_input() -> Vec<Vec<char>> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|n| n.chars().collect())
        .collect()
}
