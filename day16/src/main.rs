use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    position: (usize, usize),
    direction: Direction,
    score: usize,
    path: HashSet<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (map, start_position, end_position) = read_input();

    let turns_map: HashMap<Direction, Vec<Direction>> = [
        (Direction::North, vec![Direction::East, Direction::West]),
        (Direction::East, vec![Direction::South, Direction::North]),
        (Direction::South, vec![Direction::West, Direction::East]),
        (Direction::West, vec![Direction::North, Direction::South]),
    ]
    .into_iter()
    .collect();

    let start_state = State {
        position: start_position,
        direction: Direction::East,
        score: 0,
        path: HashSet::from([start_position]),
    };

    let mut states = BinaryHeap::new();
    let mut checked_states_min_scores = HashMap::new();
    states.push(Reverse(start_state));

    let mut minimum_score = None;
    let mut best_paths = HashSet::new();

    while let Some(Reverse(state)) = states.pop() {
        if state.score > minimum_score.unwrap_or(usize::MAX) {
            break;
        }
        
        if state.position == end_position {
            best_paths.extend(state.path.clone());
            minimum_score = Some(state.score);
        }

        // add step to next state
        let next_position = next_position(state.position, &state.direction);
        let mut path = state.path.clone();
        path.insert(next_position);
        let next_state = State {
            direction: state.direction,
            position: next_position,
            score: state.score + 1,
            path,
        };

        if map[next_position.0][next_position.1] == Tile::Empty
            && checked_states_min_scores.get(&(next_state.position, next_state.direction)).unwrap_or(&usize::MAX) >= &next_state.score
        {
            checked_states_min_scores.insert((next_state.position, next_state.direction), next_state.score);
            states.push(Reverse(next_state));
        }

        // add turns to next state
        turns_map
            .get(&state.direction)
            .expect("next turns")
            .iter()
            .for_each(|next_direction| {
                let next_state = State {
                    direction: next_direction.clone(),
                    position: state.position,
                    score: state.score + 1000,
                    path: state.path.clone(),
                };

                if checked_states_min_scores.get(&(next_state.position, next_state.direction)).unwrap_or(&usize::MAX) >= &next_state.score
                {
                    checked_states_min_scores.insert((next_state.position, next_state.direction), next_state.score);
                    states.push(Reverse(next_state));
                }
            })
    }

    println!("{}", minimum_score.unwrap());
    println!("{}", best_paths.len());
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
