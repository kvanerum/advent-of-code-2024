use std::collections::HashSet;

fn main() {
    let input = read_input();

    let regions = split_into_regions(&input);

    // regions.iter().for_each(|(label, positions)| {
    //     println!(
    //         "{}: area={} perimeter={}",
    //         label,
    //         positions.len(),
    //         calculate_perimeter(positions, input[0].len(), input.len())
    //     )
    // });

    let result: usize = regions
        .iter()
        .map(|(_, positions)| {
            positions.len() * calculate_perimeter(positions, input[0].len(), input.len())
        })
        .sum();

    println!("{}", result);
}

fn calculate_perimeter(positions: &HashSet<(usize, usize)>, width: usize, height: usize) -> usize {
    let mut result = 0;

    positions.iter().for_each(|position| {
        // up
        if position.0 as i32 - 1 < 0 || !positions.contains(&(position.0 - 1, position.1)) {
            result += 1;
        }
        // down
        if position.0 + 1 >= height || !positions.contains(&(position.0 + 1, position.1)) {
            result += 1;
        }
        // left
        if position.1 as i32 - 1 < 0 || !positions.contains(&(position.0, position.1 - 1)) {
            result += 1;
        }
        // right
        if position.1 + 1 >= width || !positions.contains(&(position.0, position.1 + 1)) {
            result += 1;
        }
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
