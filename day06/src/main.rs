use crate::Direction::*;
use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let (start_position, objects, num_rows, num_cols) = read_input();

    let next_direction =
        HashMap::from([(North, East), (East, South), (South, West), (West, North)]);

    let (visited_positions, _) = simulate(
        start_position,
        num_rows,
        num_cols,
        &objects,
        None,
        &next_direction,
    );

    println!("{}", visited_positions.len());

    let pb = ProgressBar::new((num_rows * num_cols) as u64);

    let mut count: u16 = 0;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if (row, col) != start_position && !objects.contains(&(row, col)) {
                if let (_, true) = simulate(
                    start_position,
                    num_rows,
                    num_cols,
                    &objects,
                    Some((row, col)),
                    &next_direction,
                ) {
                    count += 1;
                }
            }
            pb.inc(1);
        }
    }

    pb.finish_with_message("Done!");

    println!("{}", count);
}

fn simulate(
    start_position: (usize, usize),
    num_rows: usize,
    num_cols: usize,
    objects: &HashSet<(usize, usize)>,
    extra_object: Option<(usize, usize)>,
    next_direction: &HashMap<Direction, Direction>,
) -> (HashSet<(usize, usize)>, bool) {
    let mut visited_positions: HashSet<(usize, usize, &Direction)> = HashSet::new();

    let mut current_row = start_position.0 as i32;
    let mut current_col = start_position.1 as i32;
    let mut direction = &North;

    while !visited_positions.contains(&(current_row as usize, current_col as usize, direction)) {
        visited_positions.insert((current_row as usize, current_col as usize, direction));

        let mut next_row = current_row;
        let mut next_col = current_col;

        match direction {
            North => next_row -= 1,
            South => next_row += 1,
            East => next_col += 1,
            West => next_col -= 1,
        }

        if !is_in_bounds(next_row, next_col, num_rows, num_cols) {
            return (remove_direction(&visited_positions), false);
        }

        let next_position = (next_row as usize, next_col as usize);

        if objects.contains(&next_position) || extra_object == Some(next_position) {
            direction = next_direction.get(&direction).expect("next direction");
        } else {
            current_row = next_row;
            current_col = next_col;
        }
    }

    (remove_direction(&visited_positions), true)
}

fn is_in_bounds(row: i32, column: i32, num_rows: usize, num_cols: usize) -> bool {
    row >= 0 && row < num_rows as i32 && column >= 0 && column < num_cols as i32
}

fn remove_direction(
    visited_positions: &HashSet<(usize, usize, &Direction)>,
) -> HashSet<(usize, usize)> {
    visited_positions
        .iter()
        .map(|&(row, col, _)| (row, col))
        .collect()
}

fn read_input() -> ((usize, usize), HashSet<(usize, usize)>, usize, usize) {
    let mut start_position = None;
    let mut objects = HashSet::new();

    let map: Vec<Vec<char>> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    map.iter().enumerate().for_each(|(row_index, line)| {
        line.iter().enumerate().for_each(|(col_index, &ch)| {
            if ch == '#' {
                objects.insert((row_index, col_index));
            } else if ch == '^' {
                start_position = Some((row_index, col_index));
            }
        })
    });

    (
        start_position.expect("start position"),
        objects,
        map.len(),
        map[0].len(),
    )
}
