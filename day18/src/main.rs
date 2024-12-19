use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = read_input();
    let grid_size = 70;

    let mut shortest_path = calculate_shortest_path(grid_size, &input[0..1024].iter().collect());

    println!("{}", shortest_path.expect("shortest path length"));

    shortest_path = calculate_shortest_path(grid_size, &HashSet::new());

    let mut bytes_fallen = 0;
    while shortest_path.is_some() {
        bytes_fallen += 1;
        shortest_path =
            calculate_shortest_path(grid_size, &input[0..=bytes_fallen].iter().collect());
    }

    println!("{},{}", input[bytes_fallen].1, input[bytes_fallen].0);
}

fn calculate_shortest_path(
    grid_size: usize,
    fallen_bytes: &HashSet<&(usize, usize)>,
) -> Option<usize> {
    let mut queue = VecDeque::from([((0, 0), 0)]);
    let mut shortest_path_to: HashMap<(usize, usize), usize> = HashMap::new();

    while !queue.is_empty() {
        let (current_position, path_length) = queue.pop_front().unwrap();

        shortest_path_to.insert(current_position, path_length);

        if current_position == (grid_size, grid_size) {
            return Some(path_length);
        }

        let range = 0..=grid_size as i32;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|delta| {
                (
                    current_position.0 as i32 + delta.0,
                    current_position.1 as i32 + delta.1,
                )
            })
            .filter(|next_position| {
                range.contains(&next_position.0) && range.contains(&next_position.1)
            })
            .map(|next_position| (next_position.0 as usize, next_position.1 as usize))
            .filter(|next_position| !fallen_bytes.contains(&next_position))
            .for_each(|next_position| {
                if path_length + 1 < *shortest_path_to.get(&next_position).unwrap_or(&usize::MAX) {
                    queue.push_back((next_position, path_length + 1));
                    shortest_path_to.insert(next_position, path_length + 1);
                }
            });
    }

    return None;
}

fn read_input() -> Vec<(usize, usize)> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|l| l.split_once(",").expect("split"))
        .map(|(x, y)| (y.parse().expect("number"), x.parse().expect("number")))
        .collect()
}
