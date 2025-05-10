use std::collections::HashSet;

static MOVES: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let input = read_input();

    let path = create_path(&input);

    println!("{}", count_savings(100, 2, &path));
    println!("{}", count_savings(100, 20, &path));
}

fn count_savings(minimum_saving_size: usize, max_cheat_size: usize, path: &Vec<(usize, usize)>) -> usize {
    let mut count = 0;

    for from_index in 0..path.len() {
        let from = path[from_index];
        for to_index in from_index + 1..path.len() {
            let to = path[to_index];
            let time_saved = to_index - from_index;
            let dx = if from.0 > to.0 {
                from.0 - to.0
            } else {
                to.0 - from.0
            };
            let dy = if from.1 > to.1 {
                from.1 - to.1
            } else {
                to.1 - from.1
            };
            let straight_distance = dx + dy;

            if time_saved - straight_distance >= minimum_saving_size && straight_distance <= max_cheat_size
            {
                count += 1;
            }
        }
    }

    count
}

fn create_path(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut visited = HashSet::new();

    let mut iterator = find_char('S', map);
    let end_position = find_char('E', map);

    while iterator != end_position {
        path.push(iterator);
        visited.insert(iterator);

        // try all directions
        let next: Vec<(usize, usize)> = MOVES
            .iter()
            .map(|delta| calculate_next_position(iterator, delta))
            .filter(|next_position| !visited.contains(next_position))
            .filter(|next_position| map[next_position.0][next_position.1] != '#')
            .collect();

        assert_eq!(next.len(), 1);

        iterator = *next.first().unwrap();
    }

    path.push(iterator);

    path
}

fn calculate_next_position(position: (usize, usize), delta: &(i8, i8)) -> (usize, usize) {
    (
        (position.0 as i64 + delta.0 as i64) as usize,
        (position.1 as i64 + delta.1 as i64) as usize,
    )
}

fn find_char(c: char, map: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..map.len() {
        if let Some(y) = map[row].iter().position(|&r| r == c) {
            return (row, y);
        }
    }

    panic!("Didn't find char");
}

fn read_input() -> Vec<Vec<char>> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|n| n.chars().collect())
        .collect()
}
