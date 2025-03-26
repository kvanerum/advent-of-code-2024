use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let (walls, boxes, moves, start_position) = read_input();

    let move_delta_map: HashMap<Move, (i8, i8)> = [
        (Move::Up, (-1, 0)),
        (Move::Down, (1, 0)),
        (Move::Right, (0, 1)),
        (Move::Left, (0, -1)),
    ]
    .into_iter()
    .collect();

    // part 1
    let boxes_part_1 = do_moves(
        &walls,
        &mut boxes.clone(),
        moves.clone(),
        start_position,
        &move_delta_map,
        false,
    );
    println!("{}", calculate_gps_positions_part1(boxes_part_1));

    // part 2
    let expanded_walls = expand(walls);
    let mut expanded_boxes = expand(boxes);

    let boxes_part_2 = do_moves(
        &expanded_walls,
        &mut expanded_boxes,
        moves.clone(),
        (start_position.0, start_position.1 * 2),
        &move_delta_map,
        true,
    );

    println!("{}", calculate_gps_positions_part2(&boxes_part_2));
}

fn expand(positions: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut expanded = Vec::new();
    positions.iter().for_each(|(row, col)| {
        expanded.push((*row, col * 2));
        expanded.push((*row, col * 2 + 1));
    });

    expanded
}

fn do_moves(
    walls: &Vec<(usize, usize)>,
    boxes: &mut Vec<(usize, usize)>,
    moves: Vec<Move>,
    start_position: (usize, usize),
    move_delta_map: &HashMap<Move, (i8, i8)>,
    is_part_2: bool,
) -> Vec<(usize, usize)> {
    let mut current_position = start_position.clone();

    moves.iter().for_each(|m| {
        let delta = move_delta_map.get(&m).expect("move");
        let next_position = calculate_next_position(current_position, delta);

        if boxes.contains(&next_position) {
            // find all boxes that have to move
            let mut boxes_to_move = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(next_position);
            if is_part_2 {
                queue.push_back((
                    next_position.0,
                    find_other_part_of_box(next_position, boxes),
                ));
            }

            while let Some(position) = queue.pop_front() {
                boxes_to_move.insert(position);
                let next_box_position = calculate_next_position(position, delta);

                if boxes.contains(&next_box_position) {
                    if !boxes_to_move.contains(&next_box_position) {
                        queue.push_back(next_box_position);
                    }
                    if is_part_2 {
                        let other_part_of_box = (
                            next_box_position.0,
                            find_other_part_of_box(next_box_position, boxes),
                        );
                        if !boxes_to_move.contains(&other_part_of_box) {
                            queue.push_back(other_part_of_box);
                        }
                    }
                } else if walls.contains(&next_box_position) {
                    return;
                }
            }

            //move all the boxes
            boxes.retain(|b| !boxes_to_move.contains(b));
            boxes_to_move.iter().for_each(|b| {
                boxes.push(calculate_next_position(*b, delta));
            });
            current_position = next_position;
        } else if walls.contains(&next_position) {
            return;
        } else {
            current_position = next_position;
        }
    });

    boxes.to_vec()
}

fn calculate_gps_positions_part1(boxes: Vec<(usize, usize)>) -> usize {
    boxes.iter().map(|(row, col)| 100 * row + col).sum()
}

fn calculate_gps_positions_part2(boxes: &Vec<(usize, usize)>) -> usize {
    boxes
        .iter()
        .filter(|b| find_other_part_of_box(**b, boxes) > b.1)
        .map(|(row, col)| 100 * row + col)
        .sum()
}

fn find_other_part_of_box(box_part: (usize, usize), boxes: &Vec<(usize, usize)>) -> usize {
    let is_left_part = boxes
        .iter()
        .filter(|(row, _)| *row == box_part.0)
        .map(|(_, col)| *col)
        .filter(|&col| col < box_part.1)
        .count()
        % 2
        == 0;

    if is_left_part {
        box_part.1 + 1
    } else {
        box_part.1 - 1
    }
}

fn calculate_next_position(position: (usize, usize), delta: &(i8, i8)) -> (usize, usize) {
    (
        (position.0 as i64 + delta.0 as i64) as usize,
        (position.1 as i64 + delta.1 as i64) as usize,
    )
}

fn read_input() -> (
    Vec<(usize, usize)>,
    Vec<(usize, usize)>,
    Vec<Move>,
    (usize, usize),
) {
    let parts = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .collect::<Vec<_>>();

    let mut walls = Vec::new();
    let mut boxes = Vec::new();

    let mut start_position: Option<(usize, usize)> = None;
    parts[0].split("\n").enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(column, c)| {
            if c == '@' {
                start_position = Some((row, column));
            } else if c == '#' {
                walls.push((row, column));
            } else if c == 'O' {
                boxes.push((row, column));
            }
        })
    });

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

    (
        walls,
        boxes,
        moves,
        start_position.expect("starting position"),
    )
}
